# Solana Todo dApp

A decentralized Todo application built on Solana blockchain using Anchor framework. Users can create tasks, mark them as complete, and manage their todo lists on-chain.

## Features

- Create todo tasks with name and description
- Mark tasks as complete
- Unique task names
- Maximum 10 tasks per user
- Secure ownership validation
- On-chain storage

## Prerequisites

- Node.js 14+ and npm
- Rust and Cargo
- Solana CLI tools
- Anchor Framework
- Phantom Wallet (or other Solana wallet)

## Installation

1. Clone the repository
```bash
npx create-solana-dapp todo
cd todo
```

2. Install dependencies
```bash
npm install
```

3. Build the program
```bash
anchor build
```

4. Get the program ID and update it in `lib.rs` and `Anchor.toml`:
```bash
solana address -k target/deploy/todo-keypair.json
```

Replace the program ID in:
- `programs/todo/src/lib.rs`: `declare_id!("your-program-id")`
- `Anchor.toml`: `todo = "your-program-id"`

## Project Structure

```
todo/
├── app/          # Next.js frontend
│   ├── components/
│   ├── pages/
│   └── package.json
│
├── anchor/       # Solana program
│   └── programs/     # Solana program
│       └── todo/
│           └── src/
│               └── lib.rs
│   └── Anchor.toml
```

## Program Constraints

- Task name: 4-20 characters
- Task description: maximum 180 characters
- Maximum 10 tasks per user
- Minimum 0.0001 SOL required for initialization

## Development

1. Start local validator:
```bash
solana-test-validator
```

2. Deploy the program:
```bash
anchor deploy
```

3. Start the frontend:
```bash
cd app
npm run dev
```

## Testing

Run the test suite:
```bash
anchor test
```

## Frontend Development

The frontend is built with Next.js and includes:
- Wallet connection
- Task creation form
- Task list display
- Task completion functionality

To modify the frontend:
1. Navigate to `app/` directory
2. Install dependencies: `npm install`
3. Run development server: `npm run dev`
4. Access at: `http://localhost:3000`

## Program Instructions

### Initialize
Creates a new todo list for a user
```typescript
await program.methods
  .initialize()
  .accounts({...})
  .rpc();
```

### Add Task
Adds a new task to the list
```typescript
await program.methods
  .addTask(name, description)
  .accounts({...})
  .rpc();
```

### Complete Task
Marks a task as complete
```typescript
await program.methods
  .taskComplete(taskIndex)
  .accounts({...})
  .rpc();
```

## Error Codes

- `NameTooLong`: Task name exceeds 20 characters
- `NameTooShort`: Task name less than 4 characters
- `NameAlreadyExists`: Duplicate task name
- `DescriptionTooLong`: Description exceeds 180 characters
- `TaskNotFound`: Invalid task index
- `MaxTasksReached`: Exceeded 10 tasks limit
- `TaskAlreadyCompleted`: Task already marked complete
- `InsufficientFunds`: Not enough SOL to create todo list

## Security Considerations

- PDA-based account creation
- Owner-only task management
- Input validation
- Space limitation checks
- Duplicate name prevention

## Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Push to branch
5. Create Pull Request

## License

This project is licensed under the MIT License.
