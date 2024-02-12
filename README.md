# AWS Lambda function to reverse a name
### Divya Sharma (ds655)


## Description

This project is a Rust application that is designed to run on AWS Lambda. It takes your name and returns the reverse of your name. The application is deployed to AWS Lambda using the AWS CLI.

## Requirements

- Rust: 1.76.0
- AWS CLI: [specify version]
- An AWS account

## Setup

1. **Compile the Rust application**:

    ```bash
    cargo build --release
    ```

2. **Create the `bootstrap` file**:

    Create a new file named `bootstrap` with the following content:

    ```bash
    #!/bin/sh
    ./my_lambda_function
    ```

    Replace `my_lambda_function` with the name of your compiled Rust application.

    Make the `bootstrap` file executable:

    ```bash
    chmod +x bootstrap
    ```

3. **Create the deployment package**:

    Add the `bootstrap` file and your compiled Rust application to a zip file:

    ```bash
    zip rust.zip bootstrap ./target/release/my_lambda_function
    ```

4. **Upload the deployment package to AWS Lambda**:

    Use the AWS CLI to upload the zip file to AWS Lambda:

    ```bash
    aws lambda update-function-code --function-name my_lambda_function --zip-file fileb://rust.zip
    ```

    Replace `my_lambda_function` with the name of your AWS Lambda function.

## Running the Application

To run the application, invoke the AWS Lambda function:

```bash
aws lambda invoke --function-name my_lambda_function outputfile.txt