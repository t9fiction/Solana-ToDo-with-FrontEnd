#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

// Program constants
const MAX_TODO_ITEMS: usize = 10;
const MIN_VECTOR_NAME_LENGTH: usize = 4;
const LAMPORTS_PER_TODO: u64 = 100_000; // 0.0001 SOL minimum required

#[program]
pub mod todo {
    use super::*;

    /// Initialize a new todo list for a user
    /// Creates a PDA account to store tasks
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let todo = &mut ctx.accounts.todo;
        todo.tasks = Vec::new();
        todo.user = ctx.accounts.user.key();
        msg!("Todo list initialized for user: {}", todo.user);
        Ok(())
    }

    /// Add a new task to the todo list
    /// Validates name and description lengths
    /// Checks for duplicate names and max tasks limit
    pub fn add_task(ctx: Context<AddTask>, name: String, description: String) -> Result<()> {
        // Input validation
        if name.len() > 20 {
            msg!("Name too long: {}", name.len());
            return Err(error!(TodoError::NameTooLong));
        }
        if description.len() > 180 {
            msg!("Description too long: {}", description.len());
            return Err(error!(TodoError::DescriptionTooLong));
        }
        if name.len() < MIN_VECTOR_NAME_LENGTH {
            msg!("Name too short: {}", name.len());
            return Err(error!(TodoError::NameTooShort));
        }

        let todo = &mut ctx.accounts.todo;

        // Check task limit
        if todo.tasks.len() >= MAX_TODO_ITEMS {
            msg!("Maximum tasks reached: {}", MAX_TODO_ITEMS);
            return Err(error!(TodoError::MaxTasksReached));
        }

        // Check for duplicate names
        for task in &todo.tasks {
            if task.name == name {
                msg!("Task name already exists: {}", name);
                return Err(error!(TodoError::NameAlreadyExists));
            }
        }

        // Create and add new task
        let task = TodoItem {
            name,
            description,
            completed: false,
        };
        todo.tasks.push(task);
        msg!("New task added successfully");
        Ok(())
    }

    /// Mark a task as complete
    /// Validates task index and completion status
    pub fn task_complete(ctx: Context<CompleteTask>, task_index: u8) -> Result<()> {
      let todo = &mut ctx.accounts.todo;
      
      // Validate task index
      if (task_index as usize) >= todo.tasks.len() {
          msg!("Task index out of bounds: {}", task_index);
          return Err(error!(TodoError::TaskNotFound));
      }

      // Check if already completed
      if todo.tasks[task_index as usize].completed {
          msg!("Task already completed: {}", task_index);
          return Err(error!(TodoError::TaskAlreadyCompleted));
      }

      // Mark task as complete
      todo.tasks[task_index as usize].completed = true;
      msg!("Task marked as complete: {}", task_index);
      Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 4 + (TodoItem::INIT_SPACE * MAX_TODO_ITEMS), // 8 discriminator + 32 pubkey + 4 vec len + items
        seeds = [b"todo", user.key().as_ref()],
        bump,
        constraint = user.lamports() >= LAMPORTS_PER_TODO @ TodoError::InsufficientFunds
    )]
    pub todo: Account<'info, Todo>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct AddTask<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"todo", user.key().as_ref()],
        bump,
        has_one = user,
        constraint = todo.tasks.len() < MAX_TODO_ITEMS @ TodoError::MaxTasksReached
    )]
    pub todo: Account<'info, Todo>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"todo", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub todo: Account<'info, Todo>,
}

#[account]
#[derive(Debug)]
pub struct Todo {
    pub user: Pubkey,  // 32 bytes
    pub tasks: Vec<TodoItem>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct TodoItem {
    #[max_len(20)]
    name: String,
    #[max_len(180)]
    description: String,
    completed: bool,
}

#[error_code]
pub enum TodoError {
    #[msg("The provided name should be 20 characters long maximum.")]
    NameTooLong,
    #[msg("The provided name should be at least four characters long.")]
    NameTooShort,
    #[msg("The provided name already exists.")]
    NameAlreadyExists,
    #[msg("The provided description should be 180 characters long maximum.")]
    DescriptionTooLong,
    #[msg("The specified task was not found.")]
    TaskNotFound,
    #[msg("The maximum number of tasks has been reached.")]
    MaxTasksReached,
    #[msg("This task is already marked as completed.")]
    TaskAlreadyCompleted,
    #[msg("Insufficient funds to create todo list.")]
    InsufficientFunds,
}