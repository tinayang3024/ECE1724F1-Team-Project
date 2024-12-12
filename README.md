# Personal Finance Tracker

## Team Members

- Dingyu Yang (1003802183 | dingyu.yang@mail.utoronto.ca) 
- Sophie Mao (1004166957 | sophie.mao@mail.utoronto.ca)
- Renli Zhang (1005828339 | renli.zhang@mail.utoronto.ca)

## Motivation
The motivation behind choosing a Personal Finance Tracker as our team project stems from the desire to create a light-weight yet practical tool that can genuinely improve users' financial literacy and management skills. We recognized a gap in the Rust ecosystem for a user-friendly command-line utility that helps individuals track their income and expenses effectively.

By creating this utility, we aim to assist users in budgeting, tracking expenses, setting and achieving financial goals, and managing debt, all while providing insights and trend visualizations. Rust's strengths in data security and concurrency make it an ideal choice for a finance tool, ensuring that user information remains safe and processing is efficient compared to applications built in other languages. 

Moreover, this project offers a unique opportunity for us to build an application from scratch, encompassing both frontend UI and backend server components, along with a database. We will delve into various Rust libraries, gaining hands-on experience that fosters a holistic understanding of full-stack development—from UI/UX design to server-side logic and database management. Managing this project from inception to deployment will not only enhance our technical skills but also teach us essential lessons in planning, designing, implementing, testing, and ultimately delivering robust software solutions.

## Objective and key features
### Objective
The Personal Finance Tracker is a command-line utility designed to help users effectively manage their basic financial needs and achieve their financial goals. The program includes a backend server that communicates with a structured PostgresSQL database for recorded financial information and an easy-to-navigate user interface for interaction, and it aims to provide users an efficient and secure tool to track their financial activities.


The user interface shall support transaction logging in a structured, organized manner, allowing users to manage different bank accounts under one profile and easily filter through historic records with a given time range or categories. Upon user query, this Personal Finance tracker shall provide a comprehensive overview of one’s financial status as well as a simple and efficient financial managing experience for users.


The data storage shall support a sufficient size for this multi-user application, while still allowing queries to be executed in a timely fashion. There shall also be a scalable data transfer mechanism between the user interface and the storage.


Our tracker provides a novel view to the Rust ecosystem. While Rust is known for a wide range of applications, from web development to data analysis to artificial intelligence, the ecosystem currently lacks a simple and efficient command-line utility for personal finance tracking. The Personal Finance Tracker is designed to be easy to use, customizable, with all data stored locally which emphasizes privacy and security. This tool will be great for users who want to maintain full control over their information, and having a handy, practical tool on their local systems




### Key Features
#### Transaction Logging


The main feature of the tracker is transaction logging. Every time a user has a financial transaction activity, they can log it into the system with details such as type (expense or income), categories (rent, grocery, paycheck, etc), the transaction amount and an optional description. This will allow users to keep a very detailed record of transaction history, making it easier to manage and analyze. Additionally, users will be able to edit a transaction, revising potential errors while keeping all the information up to date. Transactions will be organized based on the account that transaction belongs to, currently account types of either chequing or credit will be supported.


#### Categorizing and filtering


One of our focus of the Personal Finance Tracker is to assist users with easier finance records filtering and categorizing. Financial transactions are rarely uniform and often hard to find a pattern of. Our tracker takes that into consideration and allows users to categorize each transaction according to a set of customizable categories that fits their unique needs. Additionally, users can provide a description along with the transaction for future reference. 


Furthermore, in order for users to get an overview of their historic financial activities and have a better understanding of their financial status, the tracker provides a way for users to filter through their records by type, categories or any combinations of these fields. Users need to specify the account they want to check together with any of the filtering options, and they will get a sum of total amount spent and a list of all transactions as the query output. This feature is particularly valuable for users to organize transaction records with customized preferences, compare the amount horizontally across different categories or through time periods. It’s a great way for users to better understand their financial habits and make any adjustments that fits to their financial goals.

#### Account Security


Our goal is to make sure the finance tracker is reliable and secure. Each user will need to create their profile with a unique username, and will be given a unique account id when registering a new account or transaction id when recording a new transaction. The unique ids should be confidential to a person and will keep each user profile secure. Users will also be able to delete their profile from a tracker if needed. 

#### Data Storage

The Personal Finance tracker will adopt a SQL-based back-end database for storing user information, as SQL provides an efficient framework for database management and information query. The user information will be organized into three main database relations, one for user level information, one for account level information, and one for transaction level information.


To communicate with the back-end database, the Personal Finance tracker will need a mechanism to transfer data from the front-end user interface to the back-end. Although for the scope of this project the team will not look into hosting the Personal Finance tracker as a web application, the communication between the user interface and the data storage shall still be robust and scalable so that it can be easily transformed into a modern web application outside of this project when the time and financial limits are uplifted. To achieve the scalability described above, the Personal Finance tracker has adopted a HTTPS protocol with a reqwest-based client and a actix-web-based server for the data transfer between the front-end and the back-end, with the server hosted locally for simplicity.

## Reproducibility Guide
1. Clone the Repository
```bash
git clone https://github.com/tinayang3024/ECE1724F1-Team-Project.git
cd ECE1724F1-Team-Project
```

2. Navigate to the server side and start server
```bash
cd server
cargo build
cargo run
```

3. Navigate to the client side and start frontend
```bash
cd client
cargo build
cargo run
```

The tool should be ready to use. Please expand the terminal running the client program to full screen to ensure the UI is displayed properly.


## User's (or Developer’s) Guide

### User's Guide

Users can perform the following operations through the frontend interface. Please follow the set up instructions in Reproducibility Guide section below to run both the server and client components. Ensure server is running before running clients to start the tool.

#### Create a new user / view existing user information

After running `cargo run` in the client side, the following page should be shown
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/0bb13a1d-81b0-4457-a096-151360906e97" />
Press `e` and type in an example username `test`
<img width="1470" alt="2" src="https://github.com/user-attachments/assets/59218e0b-f223-4828-bf36-1ac74e46d005" />
Press `enter` to create the user, or view existing user information if the username is already registered

#### Create a new account

After pressing `enter` from the user page, the following page should be shown
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/37247490-1475-4eda-8204-6fe222e6cea2" />
Press `a` to create an account, press `e`, type and press `enter` for each fields of the account information
<img width="1470" alt="3" src="https://github.com/user-attachments/assets/435ed97d-a8d2-47e8-9a5f-b30b0029aac0" />
Press `enter` again to create the account, you should see the created account showing up in the "Associated Accounts" panel under "Profile Data"
<img width="1470" alt="4" src="https://github.com/user-attachments/assets/8d40747c-647c-46ba-bd29-821156dfc220" />

####  Update existing account name or limit

To update existing account information, first press `l` from the last page, you should see the account light up like below, press `enter` to select this account
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/9cb90e0a-6d2d-4cb4-83ed-c02d3cf8ab15" />
press `esc` to exit account selection mode, then use `up` or `down` key to select the field of the account information to be changed, in this example, "Card limit" field is selected
<img width="1470" alt="2" src="https://github.com/user-attachments/assets/88c120fb-3991-4f64-902d-ac100f767880" />
press `e`, update the field, press `enter`, then `enter` again to update the field
<img width="1470" alt="3" src="https://github.com/user-attachments/assets/a6493964-d3ee-457d-9de7-0591a1f1359f" />

#### Log a new transaction record

From the last step, press `t` to create a new transaction
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/f58bff60-f517-44f7-8e0c-a737405c4e50" />
Press `e`, type and press `enter` for each fields of the transaction information
<img width="1470" alt="2" src="https://github.com/user-attachments/assets/bacab5bd-d657-42a5-8a74-f20529175225" />
Press `enter` again to create the transaction, you should see the created transaction showing up in the "Transaction Records" section under "Account Details of 1"
<img width="1470" alt="3" src="https://github.com/user-attachments/assets/223d5ed4-91cb-4158-8fb7-9e1da7c18a9f" />
Press `esc` to exit transaction selection mode


#### View/Updating existing transaction information

From last step, press `s` to enter transaction selection mode, then press `enter` to select the transaction we created above, that should take you back to a page similar as the transaction creation page, just press `e` and `enter` for the fields you would like to modify, then `enter` again when everything is done
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/2d6d9479-0a5b-4771-a4e3-28c91249ccaa" />
Press `esc` to exit transaction selection mode

#### Transaction filtering

From last step, first press `esc` to exit transaction selection mode. We will create three more transactions following the instruction in [Log a new transaction record](#log-a-new-transaction-record) with the following information:

  - Transaction Description: Dinner, Transaction Type: Expenses, Transaction Amount: 24.31, Transaction Category: Meal
  - Transaction Description: TTC, Transaction Type: Expenses, Transaction Amount: 3.2, Transaction Category: Transit
  - Transaction Description: Transaction, Transaction Type: Income, Transaction Amount: 500, Transaction Category: Pay

The transaction page should look like the following now
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/e057087a-ea4f-45a5-bdc5-de642479696d" />
Use `up` and `down` keys to select the filter option ("Transaction Type" or "Transaction Category"), in this example, we will filter by "Transaction Category" with "Meal", but feel free to experiment more other filtering options. Press `e`, enter "Meal", then `enter`, and `enter` again to apply the filtering, the "Transaction Records" and "Balance" fields should reflect the filtering
<img width="1470" alt="2" src="https://github.com/user-attachments/assets/4e98ea1f-bb99-4114-86ce-4e9a656b85a1" />


#### Delete a transaction

Press `s` to enter transaction selection mode
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/483d50a3-f04a-4085-8178-44889def14ea" />
Then `enter` on the transaction you would like to delete, for example, transaction for "Lunch"
<img width="1470" alt="2" src="https://github.com/user-attachments/assets/8feecb37-c0ef-4099-b501-f51d2f8cce69" />
Press `d` to delete the selected transaction, you should end up in the accounts page like below
<img width="1470" alt="3" src="https://github.com/user-attachments/assets/2a4ff2a7-f1d3-46b0-9853-be7f7422c70a" />
Press `esc` to exit transaction selection mode


#### Delete an account

In the account page, press `d` to delete the current account. You should end up with a page like the following:
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/a78c3085-4230-43de-950f-777f17b9c58c" />

**Note**: If there are multiple accounts, press `l` to enter account selection mode, select the account to be deleted with `up` and `down` keys, press `enter`, then press `d`

#### Delete a user

From last step, press `b` to delete the user, you should return to the user creation page in the first step
<img width="1470" alt="1" src="https://github.com/user-attachments/assets/03d74a32-88f3-4589-9177-c4a693656c86" />


### Developer's Guide

#### Server API endpoints
Developers can use these endpoints to integrate the server with other clients or use curl to call the APIs. All create and query APIs will return results in JSON format, and all deleting record APIs will return a `200 OK` status upon successful operation. The following operations require the server to be started, please refer to step 1 and 2 from the [Reproducibility Guide](#reproducibility-guide). **Note**: Following the below example curl commands in the listed sequence may not always succeed as the form data are randomly chosen and does not reflect the database status at the moment.

- User API

  - Create a new user if the given username does not exist in the database, otherwise return existing user accounts.

    - URL: `/query_or_create_user`

    - METHOD: POST
   
    - RETURNS: A list of accounts associated with the user

    - Example Request Body (as `web::Form`): 
      ```rust 
      username=Renli Zhang
      ```

    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/query_or_create_user -X POST -d "username=Renli Zhang"
      ```
    

  - Delete a user

    - URL: `/delete_user`

    - METHOD: POST

    - Example Request Body (as `web::Form`): 
      ```rust 
      username=Renli Zhang
      ```

    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/delete_user -X POST -d "username=Renli Zhang"
      ```

- Account API

  - Create a new account or update existing account name or limit, username must exist in the database for this query to succeed

    - URL: `/create_or_update_account`

    - METHOD: POST
   
    - RETURNS: The account id of the created/updated account

    - Example Request Body (as `web::Form`): 
      ```rust 
      username=Renli Zhang&account_name=First account&account_type=Chequing&account_limit=2000
      ```
      
    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/create_or_update_account -X POST -d "username=Renli Zhang&account_name=First account&account_type=Chequing&account_limit=2000"
      ```

  - Delete an account, account id can be found by querying user information, or the id returned by account creation
    
    - URL: `/delete_account/{id}`
    
    - METHOD: GET
   
    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/delete_account/1
      ```

- Transaction API

  - Create a new transaction record or update existing transaction information, account id must exist in the database for this query to succeed
    
    - URL: `/create_or_update_transaction`
    
    - METHOD: POST
   
    - RETURNS: The transaction id of the created/updated transaction

    - Example Request Body (as `web::Form`): 
    ```rust 
    transaction_date=2024-11-28&transaction_type=Income&category=work&amount=500&transaction_memo=first payment&account_id=1
    ```

    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/create_or_update_transaction -X POST -d "transaction_date=2024-11-28&transaction_type=Income&category=work&amount=500&transaction_memo=first payment&account_id=1"
      ```

  - Delete a transaction, transaction id can be found by querying the accounts, or the id returned by transaction creation
    
    - URL: `/delete_transaction/{id}`
    
    - METHOD: GET
   
    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/delete_transaction/1
      ```

- View Records API

  - Query user's financial records by account, transaction type or category, account id must exist in the database for this query to succeed
    
    - URL: `/query_account`
    
    - METHOD: POST
      
    - RETURNS: A list of transaction associated with the provided account id that satisfy the given filter

    - Example Request Body (as `web::Form`): 
      ```rust 
      transaction_type=Income&category=work&account_id=1
      ```
    - Example curl command (assuming hosted locally):
      ```
      curl http://localhost:8080/query_account -X POST -d "transaction_type=Income&category=work&account_id=1"
      ```

## Video Demo

Video Demo can be found in issue https://github.com/tinayang3024/ECE1724F1-Team-Project/issues/1.

## Contributions by each team member

<table class="tg">
  <thead>
    <tr>
      <th>Project Phase</th>
      <th>Duration</th>
      <th>Tasks</th>
      <th>Owner</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan=3>Basic system setup</td>
      <td rowspan=3>2 weeks</td>
      <td>Backend database setup</td>
      <td>Sophie</td>
    </tr>
    <tr>
      <td>HTTPS server setup (ensure the communication between database and CLI)</td>
      <td>Renli</td>
    </tr>
    <tr>
      <td>Basic user interface (CLI) setup</td>
      <td>Tina</td>
    </tr>
    <tr>
      <td rowspan=3>API implementation</td>
      <td rowspan=3>2 weeks</td>
      <td>Server API Handler</td>
      <td>Renli</td>
    </tr>
    <tr>
      <td>Database Query API</td>
      <td>Sophie</td>
    </tr>
    <tr>
      <td>Frontend API Call</td>
      <td>Tina</td>
    </tr>
    <tr>
      <td rowspan=2>Final integration and testing</td>
      <td rowspan=2>1 week</td>
      <td>Testing</td>
      <td>Team</td>
    </tr>
    <tr>
      <td>Integration</td>
      <td>Team</td>
    </tr>
  </tbody>
</table>

## Lessons learned and concluding remarks

By building the Personal Finance Tracker project from scratch, we gained practical experience working with Rust and learned how to design and develop a well-structured application leveraging Rust's robust ecosystem. We explored several useful crates, including `actix_web` for server development, `sqlx` for database interactions, `crossterm` for creating text-based user interfaces, and `Ratatui` for building an efficient and user-friendly UI. Additionally, we honed our teamwork and problem-solving skills through collaboration and joint debugging sessions. Group brainstorming sessions allowed us to assist one another in overcoming challenges and improving the overall quality of our project.
