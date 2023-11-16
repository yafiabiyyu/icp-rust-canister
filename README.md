
## About

I developed a job portal for this project using the Rust programming language, simplifying the process of creating and deploying canisters on the Internet Computer. This initiative served as a hands-on exercise aimed at enhancing my proficiency in navigating the Internet Computer platform. The project showcases my skills in Rust programming, particularly in crafting decentralized applications tailored for the unique environment of the Internet Computer.

## Setup

1. Clone Project
    ```bash
    git clone git@github.com:yafiabiyyu/icp-rust-canister.git

    cd icp-rust-canister
    ```

2. Start the IC local development environment
    ```
    dfx start --background --clean
    ```

3. Deploy the canisters to the local development environment
    ```
    npm run gen-deploy
    ```