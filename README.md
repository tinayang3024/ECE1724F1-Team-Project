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
The Personal Finance Tracker is a command-line utility designed to help users effectively manage their basic financial needs and achieve their financial goals. The tool aims to provide an easy solution for users to track their incomes and expenses through an easy-to-navigate user interface and a structured storage for the recorded financial information.


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


Our goal is to make sure the finance tracker is reliable and secure. Each user will need to create their profile with a unique username, and will be given a unique account id when registering a new account. They will need to use the combination of the username and account id for performing each action within the tracker. This username-account-id combination prevents users from accidentally making modifications to wrong accounts and keeps each user profile secure. Users will also be able to delete their profile from a tracker if needed. 

#### Data Storage

The Personal Finance tracker will adopt a SQL-based back-end database for storing user information, as SQL provides an efficient framework for database management and information query. The user information will be organized into three main database relations, one for user level information, one for account level information, and one for transaction level information.


To communicate with the back-end database, the Personal Finance tracker will need a mechanism to transfer data from the front-end user interface to the back-end. Although for the scope of this project the team will not look into hosting the Personal Finance tracker as a web application, the communication between the user interface and the data storage shall still be robust and scalable so that it can be easily transformed into a modern web application outside of this project when the time and financial limits are uplifted. To achieve the scalability described above, the Personal Finance tracker will adopt a HTTPS protocol for the data transfer between the front-end and the back-end.


## Tentative Plan

Overall, the project implementation will be divided into three phases: basic system setup, basic user query implementation, and final testing and integration. Each of these phases will contain sub-tasks that are allocated to each team member.

The basic system setup phase includes building the backbone system for the personal finance tracker: setting up the backend SQL-based database, the HTTPS server that communicates frontend user query to the backend database, and the frontend CLI user interface. Two weeks are allocated to this phase, during which the team members will familiarize themselves with the crates corresponding to the feature they are responsible for, implement the backbone features, and perform a basic integration of these features.

The basic user query implementation phase includes implementing the mandatory transaction requests that users will use to modify or query their personal finance data. Those queries are divided into three categories: user-related queries, account-related queries, and transaction related queries. Two weeks are allocated to this phase.

The final testing and integration phase will not be further divided into subtasks, and all team members will work on this task together to review each other’s code as well as testing the system’s functionality and performance against the design expectation outlined in this proposal. One week is allocated to this phase.

The following table summarizes the development plan, and the responsibility assignment to each of the team members:
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
      <td rowspan=3>Basic user query implementation</td>
      <td rowspan=3>2 weeks</td>
      <td>User-related queries</td>
      <td>Renli</td>
    </tr>
    <tr>
      <td>Account-related queries</td>
      <td>Sophie</td>
    </tr>
    <tr>
      <td>Transaction-related queries</td>
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

The above totals to five weeks of planned development, and the team has allocated an extra week for contingency planning and documentation needs. If the implementation goes according to or beyond the above outline, the team will use the extra time to implement any advanced user queries not included in the key features section, such as providing a budgeting report, generating a customizable transaction report including histograms of income and expenses pattern, and a savings account type in addition to the chequing and credit account type supported as base features.
