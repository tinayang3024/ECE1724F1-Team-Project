# Personal Finance Tracker

## Team Members

- Dingyu Yang (1003802183) 
- Sophie Mao (1004166957)
- Renli Zhang (1005828339)

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


The main feature of the tracker is transaction logging. Every time a user has a financial transaction activity, they can log it into the system with details such as type (expense or income), categories (rent, grocery, paycheck, etc), the transaction amount and an optional description. This will allow users to keep a very detailed record of transaction history, making it easier to manage and analyze. The tracker would also check for the remaining balance before performing a transaction and return a warning if there is no sufficient balance in the account. Additionally, users will be able to edit a transaction, revising potential errors while keeping all the information up to date. Transactions will be organized based on the account that transaction belongs to, currently account types of either chequing or credit will be supported.


In Addition to external transaction logging, a novel feature of the tracker is enabling transactions internally between different accounts under a user profile. After creating a profile in the tracker, users will be able to register accounts they have with the amount if chequing or the credit limit if credit. The tracker offers a way for users to make internal transactions by specifying the source and destination account id, and the tracker will ensure the reconciliation of accounts and checks for sufficient balance to complete the transaction. This way users don’t need to manually enter the transaction separately for each account, which prevents any human errors, keeps the data accurate and provides a simple, friendly user-experience. Users will also be able to change their credit limits in the tracker to reflect their most up-to-date information.


#### Categorizing and filtering


One of our focus of the Personal Finance Tracker is to assist users with easier finance records filtering and categorizing. Financial transactions are rarely uniform and often hard to find a pattern of. Our tracker takes that into consideration and allows users to categorize each transaction according to a set of customizable categories that fits their unique needs. Additionally, users can provide a description along with the transaction for future reference. 


Furthermore, in order for users to get an overview of their historic financial activities and have a better understanding of their financial status, the tracker provides a way for users to filter through their records by time range, type, categories or any combinations of these fields. Users need to specify the account they want to check together with any of the filtering options, and they will get a sum of total amount spent and a list of all transactions as the query output. This feature is particularly valuable for users to organize transaction records with customized preferences, compare the amount horizontally across different categories or through time periods. It’s a great way for users to better understand their financial habits and make any adjustments that fits to their financial goals.

#### Account Security


Our goal is to make sure the finance tracker is reliable and secure. Each user will need to create their profile with a unique username, and will be given a unique account id when registering a new account or transaction id when recording a new transaction. They will need to enter the unique id for performing each action within the tracker. The unique ids should be confidential to a person and will keep each user profile secure. Users will also be able to delete their profile from a tracker if needed. 

#### Data Storage

The Personal Finance tracker will adopt a SQL-based back-end database for storing user information, as SQL provides an efficient framework for database management and information query. The user information will be organized into three main database relations, one for user level information, one for account level information, and one for transaction level information.


To communicate with the back-end database, the Personal Finance tracker will need a mechanism to transfer data from the front-end user interface to the back-end. Although for the scope of this project the team will not look into hosting the Personal Finance tracker as a web application, the communication between the user interface and the data storage shall still be robust and scalable so that it can be easily transformed into a modern web application outside of this project when the time and financial limits are uplifted. To achieve the scalability described above, the Personal Finance tracker will adopt a HTTPS protocol for the data transfer between the front-end and the back-end.

## User's (or Developer’s) Guide

### User's Guide

Users can perform the following operations through the frontend interface. Please follow the set up instructions in Reproducibility Guide section below to run both the server and client components. Ensure server is running before running clients to start the tool.
- Create a new user / view existing user information
- Create a new account / update existing account name or limit
- Log a new transaction record / updating existing transaction information
- Delete a user
- Delete an account
- Delete a transaction
- View records

### Developer's Guide

#### Server API endpoints
Developers can use these endpoints to integrate the server with other clients or use curl to call the APIs. All create and query APIs will return results in JSON format, and all deleting record APIs will return a `200 OK` status upon successful operation.

- User API

  - Create a new user or return existing user information

    URL: `/query_or_create_user`

    METHOD: POST

    Example Request Body (as `web::Form`): 
    ```rust 
    username=Renli Zhang
    ```

  - Delete a user

    URL: `/delete_user`

    METHOD: POST

    Example Request Body (as `web::Form`): 
    ```rust 
    username=Renli Zhang
    ```

- Account API

  - Create a new account or update existing account name or limit

    URL: `/create_or_update_account`

    METHOD: POST

    Example Request Body (as `web::Form`): 
    ```rust 
    username=Renli Zhang&account_name=First account&account_type=Chequing&account_limit=2000
    ```

  - Delete an account
    
    URL: `/delete_account/{id}`
    
    METHOD: GET

- Transaction API

  - Create a new transaction record or update existing transaction information
    
    URL: `/create_or_update_transaction`
    
    METHOD: POST

    Example Request Body (as `web::Form`): 
    ```rust 
    transaction_date=2024-11-28&transaction_type=Income&category=work&amount=500&transaction_memo=first payment&account_id=1
    ```

  - Delete a transaction
    
    URL: `/delete_transaction/{id}`
    
    METHOD: GET

- View Records API

  - Query user's financial records by account, transaction type or category
    
    URL: `/query_account`
    
    METHOD: POST

    Example Request Body (as `web::Form`): 
    ```rust 
    transaction_type=Income&category=work&account_id=1
    ```

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

The tool should be ready to use. Note that please ensure that the terminal running the client program is expanded to full screen to ensure the UI is displayed properly.

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
