# Medical Records Backend on Internet Computer (ICP)

This repository contains the backend smart contract for managing medical records on the Internet Computer (ICP). The smart contract allows users to register, assign roles, manage medical records, and retrieve user or medical data securely. Potential future features and improvements that could be made to the project. These developments aim to enhance the system's security, scalability, and usability while also improving its integration with other healthcare systems and creating a more user-friendly experience.

## Features

- **User Management**:
  - Register users with roles (Doctor or Patient).
  - Switch active roles for users with multiple roles.
- **Medical Records**:
  - Doctors can create medical records for patients.
  - Patients and their doctors can retrieve medical records.
- **Query Functionalities**:
  - Get information about all registered users.
  - Retrieve the active role of a user.
  - Fetch detailed medical records for a specific patient.

## Technology Stack

- **Language**: Rust
- **Framework**: Internet Computer SDK (Dfinity)
- **Candid Interface**: Used for defining the service interface.

## Getting Started

### Prerequisites

1. Install the [Dfinity SDK](https://internetcomputer.org/docs/current/developer-docs/quickstart/).
2. Ensure Rust is installed. You can get it from [Rust's official site](https://www.rust-lang.org/).
3. Set up `dfx` for local testing and deployment.

### Installation

Clone the repository:

```bash
git clone https://github.com/fxjrin/medical-records
cd medical-records
```

### Deploying Locally

1. Start the local DFX network:

   ```bash
   dfx start --background --clean
   ```

2. Deploy the canister:

   ```bash
   dfx deploy
   ```

## License

This project is licensed under the MIT License. See the LICENSE file for details.