use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    program::set_return_data,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Account {
    account_type: u8,
    start_value: u64,
    #[allow(unused)]
    payout_rate: f64,
    sale_year: u32,
    sale_value: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct MultiYearInput {
    start_year: u32,
    end_year: u32,
    pension: u64,
    salary: u64,
    revenue: u64,
    #[allow(unused)]
    invest_paid: u64,
    inflation_rates: Vec<f64>,
    invest_return_rates: Vec<f64>,
    rif_rates: Vec<f64>,
    spending: u64,
    age: u64,
    #[allow(unused)]
    downsize_year: u32,
    #[allow(unused)]
    downsize_spending: i64,
    #[allow(unused)]
    downsize_invest: i64,
    accounts: Vec<Account>,
}

#[derive(BorshSerialize, Debug)]
struct PlotDataEntry {
    year: u32,
    total_income: f32,
    remaining: f32,
    rif_end: f32,
    business_end: f32,
    invest_end: f32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let input = MultiYearInput::try_from_slice(instruction_data)?;

    let mut rif_balances: Vec<f64> = Vec::new();
    let mut business_balances: Vec<f64> = Vec::new();
    let mut invest_balances: Vec<f64> = Vec::new();
    let mut business_sold_flags: Vec<bool> = Vec::new();
    let mut business_indices: Vec<usize> = Vec::new();

    for (i, account) in input.accounts.iter().enumerate() {
        match account.account_type {
            0 => rif_balances.push(account.start_value as f64),
            1 => {
                business_balances.push(account.start_value as f64);
                business_sold_flags.push(false);
                business_indices.push(i);
            }
            3 => invest_balances.push(account.start_value as f64),
            _ => {}
        }
    }

    let mut plot_data = Vec::new();
    let mut inflation_factor = 1.0;

    for (year_idx, year) in (input.start_year..=input.end_year).enumerate() {
        let idx = year_idx / 5;
        let t = (year_idx % 5) as f64 / 5.0;
        let inflation = if idx + 1 < input.inflation_rates.len() {
            let prev = input.inflation_rates[idx];
            let next = input.inflation_rates[idx + 1];
            prev + t * (next - prev)
        } else {
            *input.inflation_rates.get(idx).unwrap_or(&0.0)
        };
        let invest_return = if idx + 1 < input.invest_return_rates.len() {
            let prev = input.invest_return_rates[idx];
            let next = input.invest_return_rates[idx + 1];
            prev + t * (next - prev)
        } else {
            *input.invest_return_rates.get(idx).unwrap_or(&0.0)
        };
        let rif_rate = if idx + 1 < input.rif_rates.len() {
            let prev = input.rif_rates[idx];
            let next = input.rif_rates[idx + 1];
            prev + t * (next - prev)
        } else {
            *input.rif_rates.get(idx).unwrap_or(&0.0)
        };

        inflation_factor *= 1.0 + inflation;
        let pension = (input.pension as f64) * inflation_factor;
        let salary = if business_sold_flags.iter().all(|&sold| sold) {
            0.0
        } else {
            (input.salary as f64) * inflation_factor
        };
        let revenue = (input.revenue as f64) * inflation_factor;
        let spending = (input.spending as f64) * inflation_factor;

        let mut total_rif_balance = 0.0;
        let mut total_rif_payout = 0.0;
        for rif_balance in &mut rif_balances {
            let rif_payout = *rif_balance * rif_rate;
            *rif_balance -= rif_payout;
            *rif_balance *= 1.0 + invest_return;
            total_rif_balance += *rif_balance;
            total_rif_payout += rif_payout;
        }

        let mut total_business_balance = 0.0;
        for (_i, (business_balance, sold)) in business_balances.iter_mut().zip(business_sold_flags.iter_mut()).enumerate() {
            if !*sold {
                *business_balance += revenue - salary;
            }
            *business_balance *= 1.0 + invest_return;
            total_business_balance += *business_balance;
        }

        let mut total_invest_balance = 0.0;
        for invest_balance in &mut invest_balances {
            *invest_balance *= 1.0 + invest_return;
            total_invest_balance += *invest_balance;
        }

        for (i, account) in input.accounts.iter().enumerate() {
            if year == account.sale_year {
                match account.account_type {
                    1 => {
                        let business_idx = business_indices.iter().position(|&idx| idx == i).unwrap();
                        if !invest_balances.is_empty() {
                            invest_balances[0] += account.sale_value as f64;
                        } else {
                            invest_balances.push(account.sale_value as f64);
                        }
                        total_invest_balance += account.sale_value as f64;
                        business_balances[business_idx] = 0.0;
                        business_sold_flags[business_idx] = true;
                        total_business_balance = business_balances.iter().sum();
                    }
                    2 => {
                        if !invest_balances.is_empty() {
                            invest_balances[0] += account.sale_value as f64;
                        } else {
                            invest_balances.push(account.sale_value as f64);
                        }
                        total_invest_balance += account.sale_value as f64;
                    }
                    _ => {}
                }
            }
        }

        for sold in &business_sold_flags {
            if *sold {
                if !invest_balances.is_empty() {
                    invest_balances[0] += revenue;
                } else {
                    invest_balances.push(revenue);
                }
                total_invest_balance += revenue;
            }
        }

        let base_annual_income = total_rif_payout + pension + salary;

        let shortfall = spending - base_annual_income;
        if shortfall > 0.0 && total_invest_balance > 0.0 {
            let withdrawal = f64::min(spending - base_annual_income, total_invest_balance);
            total_invest_balance -= withdrawal;
            if total_invest_balance < 0.0 {
                total_invest_balance = 0.0;
            }
        } else if shortfall < 0.0 {
            total_invest_balance += shortfall.abs();
        }

        let total_balance = total_rif_balance + total_business_balance + total_invest_balance;
        let remaining = total_balance;

        if base_annual_income > f32::MAX as f64 || remaining > f32::MAX as f64 || 
           total_rif_balance > f32::MAX as f64 || total_business_balance > f32::MAX as f64 || 
           total_invest_balance > f32::MAX as f64 {
            return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
        }

        // Calculate plot_data for every 5th year and the end_year
        if (year - input.start_year) % 5 == 0 || year == input.end_year {
            plot_data.push(PlotDataEntry {
                year,
                total_income: base_annual_income as f32,
                remaining: remaining as f32,
                rif_end: total_rif_balance as f32,
                business_end: total_business_balance as f32,
                invest_end: total_invest_balance as f32,
            });
        }
    }

    let serialized_plot_data = plot_data.try_to_vec()?;
    set_return_data(&serialized_plot_data);

    Ok(())
}