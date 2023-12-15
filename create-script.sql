-- Your SQL goes here
USE rustenginemysql;

CREATE TABLE TreeType
(
    tree_type_id INT PRIMARY KEY,
    type_description VARCHAR(255)
);

CREATE TABLE PeriodStatus
(
    period_status_id INT PRIMARY KEY,
    status_description VARCHAR(255)
);

CREATE TABLE PeriodType
(
    period_type_id INT PRIMARY KEY,
    type_description VARCHAR(255)
);

CREATE TABLE Company
(
    company_id INT PRIMARY KEY,
    company_name VARCHAR(255)
);

CREATE TABLE Period
(
    period_id INT PRIMARY KEY,
    period_type_id INT,
    period_name VARCHAR
(255),
    period_start_date DATETIME,
    period_end_date DATETIME,
    period_status_id INT,
    company_id INT,
    created_date DATETIME,
    modified_date DATETIME,
    created_by VARCHAR
(255),
    modified_by VARCHAR
(255),
    FOREIGN KEY
(period_type_id) REFERENCES PeriodType
(period_type_id),
    FOREIGN KEY
(period_status_id) REFERENCES PeriodStatus
(period_status_id),
    FOREIGN KEY
(company_id) REFERENCES Company
(company_id)
);

CREATE TABLE Customer
(
    customer_id INT PRIMARY KEY AUTO_INCREMENT,
    company_id INT,
    customer_type_id INT,
    customer_status_id INT,
    customer_sub_status_id INT,
    enroller_id INT,
    sponsor_id INT,
    binary_placement_id INT,
    FOREIGN KEY
(company_id) REFERENCES Company
(company_id),
    FOREIGN KEY
(enroller_id) REFERENCES Customer
(customer_id),
    FOREIGN KEY
(sponsor_id) REFERENCES Customer
(customer_id),
    FOREIGN KEY
(binary_placement_id) REFERENCES Customer
(customer_id)
);

CREATE TABLE Tree
(
    tree_id INTEGER PRIMARY KEY,
    tree_name VARCHAR
(255),
    tree_type_id INT,
    FOREIGN KEY
(tree_type_id) REFERENCES TreeType (tree_type_id),
    company_id INT,
    FOREIGN KEY
(company_id) REFERENCES Company (company_id),
    is_active BOOLEAN,
    created_date DATETIME,
    modified_date DATETIME,
    created_by VARCHAR
    (255),
    modified_by VARCHAR
    (255),
    top_node_customer_id INT,
    FOREIGN KEY
    (top_node_customer_id) REFERENCES Customer
    (customer_id)
);

CREATE TABLE CompanyTree
(
    company_id INT,
    tree_id INT,
    FOREIGN KEY (company_id) REFERENCES Company (company_id),
    FOREIGN KEY (tree_id) REFERENCES Tree (tree_id)
);

CREATE TABLE Orders
(
    order_id INT PRIMARY KEY,
    company_id INT,
    customer_id INT,
    order_status_id INT,
    order_date DATETIME,
    currency_code VARCHAR(255),
    warehouse_id INT,
    ship_method_id INT,
    order_type_id INT,
    price_type_id INT,
    first_name VARCHAR(255),
    middle_name VARCHAR(255),
    last_name VARCHAR(255),
    name_suffix VARCHAR(255),
    company VARCHAR(255),
    address1 VARCHAR(255),
    address2 VARCHAR(255),
    address3 VARCHAR(255),
    city VARCHAR(255),
    state VARCHAR(255),
    zip VARCHAR(255),
    country VARCHAR(255),
    county VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(255),
    notes VARCHAR(255),
    total FLOAT,
    sub_total FLOAT,
    tax_total FLOAT,
    shipping_total FLOAT,
    discount_total FLOAT,
    discount_percent FLOAT,
    weight_total FLOAT,
    business_volume_total FLOAT,
    commissionable_volume_total FLOAT,
    other1_total FLOAT,
    other2_total FLOAT,
    other3_total FLOAT,
    other4_total FLOAT,
    other5_total FLOAT,
    other6_total FLOAT,
    other7_total FLOAT,
    other8_total FLOAT,
    other9_total FLOAT,
    other10_total FLOAT,
    shipping_tax FLOAT,
    order_tax FLOAT,
    fed_tax_total FLOAT,
    state_tax_total FLOAT,
    fed_shipping_tax FLOAT,
    state_shipping_tax FLOAT,
    city_shipping_tax FLOAT,
    city_local_shipping_tax FLOAT,
    county_shipping_tax FLOAT,
    county_local_shipping_tax FLOAT,
    other11 VARCHAR(255),
    other12 VARCHAR(255),
    other13 VARCHAR(255),
    other14 VARCHAR(255),
    other15 VARCHAR(255),
    other16 VARCHAR(255),
    other17 VARCHAR(255),
    other18 VARCHAR(255),
    other19 VARCHAR(255),
    other20 VARCHAR(255),
    is_commissionable BOOLEAN,
    auto_order_id INT,
    return_order_id INT,
    replacement_order_id INT,
    parent_order_id INT,
    decline_count INT,
    transfer_to_customer_id INT,
    party_id INT,
    shipped_date DATETIME NULL,
    created_date DATETIME,
    locked_date DATETIME NULL,
    modified_date DATETIME NULL,
    created_by VARCHAR(255),
    modified_by VARCHAR(255),
    tax_integration_calculate BOOLEAN,
    tax_integration_commit BOOLEAN,
    handling_fee FLOAT,
    pickup_name VARCHAR(255),
    total_taxable FLOAT,
    order_sub_status_id INT,
    referral_id INT,
    FOREIGN KEY (company_id) REFERENCES Company (company_id),
    FOREIGN KEY (customer_id) REFERENCES Customer (customer_id)
);

CREATE TABLE OrderDetail
(
    order_id INT,
    order_line INT,
    order_detail_id INT,
    parent_order_detail_id INT,
    item_id INT,
    item_code VARCHAR(255),
    item_description VARCHAR(255),
    quantity INT,
    price_each FLOAT,
    price_total FLOAT,
    tax FLOAT,
    weight_each FLOAT,
    weight FLOAT,
    business_volume_each FLOAT,
    business_volume FLOAT,
    commissionable_volume_each FLOAT,
    commissionable_volume FLOAT,
    other1_each FLOAT,
    other1 FLOAT,
    other2_each FLOAT,
    other2 FLOAT,
    other3_each FLOAT,
    other3 FLOAT,
    other4_each FLOAT,
    other4 FLOAT,
    other5_each FLOAT,
    other5 FLOAT,
    original_taxable_each FLOAT,
    original_business_volume_each FLOAT,
    original_commissionable_volume_each FLOAT,
    other6_each FLOAT,
    other6 FLOAT,
    other7_each FLOAT,
    other7 FLOAT,
    other8_each FLOAT,
    other8 FLOAT,
    other9_each FLOAT,
    other9 FLOAT,
    other10_each FLOAT,
    other10 FLOAT,
    parent_item_id INT,
    taxable FLOAT,
    fed_tax FLOAT,
    state_tax FLOAT,
    city_tax FLOAT,
    city_local_tax FLOAT,
    county_tax FLOAT,
    county_local_tax FLOAT,
    manual_tax FLOAT,
    is_state_tax_override BOOLEAN,
    reference1 VARCHAR(255),
    FOREIGN KEY (order_id) REFERENCES Orders (order_id)
);

CREATE TABLE Bonus
(
    bonus_id INT PRIMARY KEY,
    bonus_name VARCHAR(255),
    bonus_percentage FLOAT,
    bonus_amount FLOAT,
    to_customer_id INT,
    source_customer_id INT,
    source_order_id INT,
    FOREIGN KEY (to_customer_id) REFERENCES Customer (customer_id),
    FOREIGN KEY (source_customer_id) REFERENCES Customer (customer_id),
    FOREIGN KEY (source_order_id) REFERENCES Orders (order_id)
);

INSERT INTO TreeType
    (tree_type_id, type_description)
VALUES
    (1, 'Enroller'),
    (2, 'Unilevel'),
    (3, 'Binary');

/* Insert 2 rows into the PeriodStatus table */
INSERT INTO PeriodStatus
    (period_status_id, status_description)
VALUES
    (1, 'Open'),
    (2, 'Closed');

-- Insert 2 rows into the PeriodType table
INSERT INTO PeriodType
    (period_type_id, type_description)
VALUES
    (1, 'Monthly'),
    (2, 'Weekly');


-- Insert 1 row into the Company table
INSERT INTO Company
    (company_id, company_name)
VALUES
    (1, 'Rust Engine');

-- Insert 24 rows into period table

INSERT INTO Period
    (period_id, period_type_id, period_name, period_start_date, period_end_date, period_status_id, company_id, created_date, modified_date, created_by, modified_by)
VALUES
    (1, 1, 'January 2023', '2023-01-01 12:00:00', '2023-01-31 11:59:59', 1, 1, '2023-01-01 12:00:00', '2023-01-01 12:00:00', 'System', 'System'),
    (2, 1, 'February 2023', '2023-02-01 12:00:00', '2023-02-28 11:59:59', 1, 1, '2023-02-01 12:00:00', '2023-02-01 12:00:00', 'System', 'System'),
    (3, 1, 'March 2023', '2023-03-01 12:00:00', '2023-03-31 11:59:59', 1, 1, '2023-03-01 12:00:00', '2023-03-01 12:00:00', 'System', 'System'),
    (4, 1, 'April 2023', '2023-04-01 12:00:00', '2023-04-30 11:59:59', 1, 1, '2023-04-01 12:00:00', '2023-04-01 12:00:00', 'System', 'System'),
    (5, 1, 'May 2023', '2023-05-01 12:00:00', '2023-05-31 11:59:59', 1, 1, '2023-05-01 12:00:00', '2023-05-01 12:00:00', 'System', 'System'),
    (6, 1, 'June 2023', '2023-06-01 12:00:00', '2023-06-30 11:59:59', 1, 1, '2023-06-01 12:00:00', '2023-06-01 12:00:00', 'System', 'System'),
    (7, 1, 'July 2023', '2023-07-01 12:00:00', '2023-07-31 11:59:59', 1, 1, '2023-07-01 12:00:00', '2023-07-01 12:00:00', 'System', 'System'),
    (8, 1, 'August 2023', '2023-08-01 12:00:00', '2023-08-31 11:59:59', 1, 1, '2023-08-01 12:00:00', '2023-08-01 12:00:00', 'System', 'System'),
    (9, 1, 'September 2023', '2023-09-01 12:00:00', '2023-09-30 11:59:59', 1, 1, '2023-09-01 12:00:00', '2023-09-01 12:00:00', 'System', 'System'),
    (10, 1, 'October 2023', '2023-10-01 12:00:00', '2023-10-31 11:59:59', 1, 1, '2023-10-01 12:00:00', '2023-10-01 12:00:00', 'System', 'System'),
    (11, 1, 'November 2023', '2023-11-01 12:00:00', '2023-11-30 11:59:59', 1, 1, '2023-11-01 12:00:00', '2023-11-01 12:00:00', 'System', 'System'),
    (12, 1, 'December 2023', '2023-12-01 12:00:00', '2023-12-31 11:59:59', 1, 1, '2023-12-01 12:00:00', '2023-12-01 12:00:00', 'System', 'System'),
    (13, 1, 'January 2024', '2024-01-01 12:00:00', '2024-01-31 11:59:59', 1, 1, '2024-01-01 12:00:00', '2024-01-01 12:00:00', 'System', 'System'),
    (14, 1, 'February 2024', '2024-02-01 12:00:00', '2024-02-29 11:59:59', 1, 1, '2024-02-01 12:00:00', '2024-02-01 12:00:00', 'System', 'System'),
    (15, 1, 'March 2024', '2024-03-01 12:00:00', '2024-03-31 11:59:59', 1, 1, '2024-03-01 12:00:00', '2024-03-01 12:00:00', 'System', 'System'),
    (16, 1, 'April 2024', '2024-04-01 12:00:00', '2024-04-30 11:59:59', 1, 1, '2024-04-01 12:00:00', '2024-04-01 12:00:00', 'System', 'System'),
    (17, 1, 'May 2024', '2024-05-01 12:00:00', '2024-05-31 11:59:59', 1, 1, '2024-05-01 12:00:00', '2024-05-01 12:00:00', 'System', 'System'),
    (18, 1, 'June 2024', '2024-06-01 12:00:00', '2024-06-30 11:59:59', 1, 1, '2024-06-01 12:00:00', '2024-06-01 12:00:00', 'System', 'System'),
    (19, 1, 'July 2024', '2024-07-01 12:00:00', '2024-07-31 11:59:59', 1, 1, '2024-07-01 12:00:00', '2024-07-01 12:00:00', 'System', 'System'),
    (20, 1, 'August 2024', '2024-08-01 12:00:00', '2024-08-31 11:59:59', 1, 1, '2024-08-01 12:00:00', '2024-08-01 12:00:00', 'System', 'System'),
    (21, 1, 'September 2024', '2024-09-01 12:00:00', '2024-09-30 11:59:59', 1, 1, '2024-09-01 12:00:00', '2024-09-01 12:00:00', 'System', 'System'),
    (22, 1, 'October 2024', '2024-10-01 12:00:00', '2024-10-31 11:59:59', 1, 1, '2024-10-01 12:00:00', '2024-10-01 12:00:00', 'System', 'System'),
    (23, 1, 'November 2024', '2024-11-01 12:00:00', '2024-11-30 11:59:59', 1, 1, '2024-11-01 12:00:00', '2024-11-01 12:00:00', 'System', 'System'),
    (24, 1, 'December 2024', '2024-12-01 12:00:00', '2024-12-31 11:59:59', 1, 1, '2024-12-01 12:00:00', '2024-12-01 12:00:00', 'System', 'System');

-- Insert 104 weeks into the period table
INSERT INTO Period
    (period_id, period_type_id, period_name, period_start_date, period_end_date, period_status_id, company_id, created_date, modified_date, created_by, modified_by)
VALUES
    (25, 2, 'Week 1 2023', '2023-01-01 12:00:00', '2023-01-07 11:59:59', 1, 1, '2023-01-01 12:00:00', '2023-01-01 12:00:00', 'System', 'System'),
    (26, 2, 'Week 2 2023', '2023-01-08 12:00:00', '2023-01-14 11:59:59', 1, 1, '2023-01-08 12:00:00', '2023-01-08 12:00:00', 'System', 'System'),
    (27, 2, 'Week 3 2023', '2023-01-15 12:00:00', '2023-01-21 11:59:59', 1, 1, '2023-01-15 12:00:00', '2023-01-15 12:00:00', 'System', 'System'),
    (28, 2, 'Week 4 2023', '2023-01-22 12:00:00', '2023-01-28 11:59:59', 1, 1, '2023-01-22 12:00:00', '2023-01-22 12:00:00', 'System', 'System'),
    (29, 2, 'Week 5 2023', '2023-01-29 12:00:00', '2023-02-04 11:59:59', 1, 1, '2023-01-29 12:00:00', '2023-01-29 12:00:00', 'System', 'System'),
    (30, 2, 'Week 6 2023', '2023-02-05 12:00:00', '2023-02-11 11:59:59', 1, 1, '2023-02-05 12:00:00', '2023-02-05 12:00:00', 'System', 'System'),
    (31, 2, 'Week 7 2023', '2023-02-12 12:00:00', '2023-02-18 11:59:59', 1, 1, '2023-02-12 12:00:00', '2023-02-12 12:00:00', 'System', 'System'),
    (32, 2, 'Week 8 2023', '2023-02-19 12:00:00', '2023-02-25 11:59:59', 1, 1, '2023-02-19 12:00:00', '2023-02-19 12:00:00', 'System', 'System'),
    (33, 2, 'Week 9 2023', '2023-02-26 12:00:00', '2023-03-04 11:59:59', 1, 1, '2023-02-26 12:00:00', '2023-02-26 12:00:00', 'System', 'System'),
    (34, 2, 'Week 10 2023', '2023-03-05 12:00:00', '2023-03-11 11:59:59', 1, 1, '2023-03-05 12:00:00', '2023-03-05 12:00:00', 'System', 'System'),
    (35, 2, 'Week 11 2023', '2023-03-12 12:00:00', '2023-03-18 11:59:59', 1, 1, '2023-03-12 12:00:00', '2023-03-12 12:00:00', 'System', 'System'),
    (36, 2, 'Week 12 2023', '2023-03-19 12:00:00', '2023-03-25 11:59:59', 1, 1, '2023-03-19 12:00:00', '2023-03-19 12:00:00', 'System', 'System'),
    (37, 2, 'Week 13 2023', '2023-03-26 12:00:00', '2023-04-01 11:59:59', 1, 1, '2023-03-26 12:00:00', '2023-03-26 12:00:00', 'System', 'System'),
    (38, 2, 'Week 14 2023', '2023-04-02 12:00:00', '2023-04-08 11:59:59', 1, 1, '2023-04-02 12:00:00', '2023-04-02 12:00:00', 'System', 'System'),
    (39, 2, 'Week 15 2023', '2023-04-09 12:00:00', '2023-04-15 11:59:59', 1, 1, '2023-04-09 12:00:00', '2023-04-09 12:00:00', 'System', 'System'),
    (40, 2, 'Week 16 2023', '2023-04-16 12:00:00', '2023-04-22 11:59:59', 1, 1, '2023-04-16 12:00:00', '2023-04-16 12:00:00', 'System', 'System'),
    (41, 2, 'Week 17 2023', '2023-04-23 12:00:00', '2023-04-29 11:59:59', 1, 1, '2023-04-23 12:00:00', '2023-04-23 12:00:00', 'System', 'System'),
    (42, 2, 'Week 18 2023', '2023-04-30 12:00:00', '2023-05-06 11:59:59', 1, 1, '2023-04-30 12:00:00', '2023-04-30 12:00:00', 'System', 'System'),
    (43, 2, 'Week 19 2023', '2023-05-07 12:00:00', '2023-05-13 11:59:59', 1, 1, '2023-05-07 12:00:00', '2023-05-07 12:00:00', 'System', 'System'),
    (44, 2, 'Week 20 2023', '2023-05-14 12:00:00', '2023-05-20 11:59:59', 1, 1, '2023-05-14 12:00:00', '2023-05-14 12:00:00', 'System', 'System'),
    (45, 2, 'Week 21 2023', '2023-05-21 12:00:00', '2023-05-27 11:59:59', 1, 1, '2023-05-21 12:00:00', '2023-05-21 12:00:00', 'System', 'System'),
    (46, 2, 'Week 22 2023', '2023-05-28 12:00:00', '2023-06-03 11:59:59', 1, 1, '2023-05-28 12:00:00', '2023-05-28 12:00:00', 'System', 'System'),
    (47, 2, 'Week 23 2023', '2023-06-04 12:00:00', '2023-06-10 11:59:59', 1, 1, '2023-06-04 12:00:00', '2023-06-04 12:00:00', 'System', 'System'),
    (48, 2, 'Week 24 2023', '2023-06-11 12:00:00', '2023-06-17 11:59:59', 1, 1, '2023-06-11 12:00:00', '2023-06-11 12:00:00', 'System', 'System'),
    (49, 2, 'Week 25 2023', '2023-06-18 12:00:00', '2023-06-24 11:59:59', 1, 1, '2023-06-18 12:00:00', '2023-06-18 12:00:00', 'System', 'System'),
    (50, 2, 'Week 26 2023', '2023-06-25 12:00:00', '2023-07-01 11:59:59', 1, 1, '2023-06-25 12:00:00', '2023-06-25 12:00:00', 'System', 'System'),
    (51, 2, 'Week 27 2023', '2023-07-02 12:00:00', '2023-07-08 11:59:59', 1, 1, '2023-07-02 12:00:00', '2023-07-02 12:00:00', 'System', 'System'),
    (52, 2, 'Week 28 2023', '2023-07-09 12:00:00', '2023-07-15 11:59:59', 1, 1, '2023-07-09 12:00:00', '2023-07-09 12:00:00', 'System', 'System'),
    (53, 2, 'Week 29 2023', '2023-07-16 12:00:00', '2023-07-22 11:59:59', 1, 1, '2023-07-16 12:00:00', '2023-07-16 12:00:00', 'System', 'System'),
    (54, 2, 'Week 30 2023', '2023-07-23 12:00:00', '2023-07-29 11:59:59', 1, 1, '2023-07-23 12:00:00', '2023-07-23 12:00:00', 'System', 'System'),
    (55, 2, 'Week 31 2023', '2023-07-30 12:00:00', '2023-08-05 11:59:59', 1, 1, '2023-07-30 12:00:00', '2023-07-30 12:00:00', 'System', 'System'),
    (56, 2, 'Week 32 2023', '2023-08-06 12:00:00', '2023-08-12 11:59:59', 1, 1, '2023-08-06 12:00:00', '2023-08-06 12:00:00', 'System', 'System'),
    (57, 2, 'Week 33 2023', '2023-08-13 12:00:00', '2023-08-19 11:59:59', 1, 1, '2023-08-13 12:00:00', '2023-08-13 12:00:00', 'System', 'System'),
    (58, 2, 'Week 34 2023', '2023-08-20 12:00:00', '2023-08-26 11:59:59', 1, 1, '2023-08-20 12:00:00', '2023-08-20 12:00:00', 'System', 'System'),
    (59, 2, 'Week 35 2023', '2023-08-27 12:00:00', '2023-09-02 11:59:59', 1, 1, '2023-08-27 12:00:00', '2023-08-27 12:00:00', 'System', 'System'),
    (60, 2, 'Week 36 2023', '2023-09-03 12:00:00', '2023-09-09 11:59:59', 1, 1, '2023-09-03 12:00:00', '2023-09-03 12:00:00', 'System', 'System'),
    (61, 2, 'Week 37 2023', '2023-09-10 12:00:00', '2023-09-16 11:59:59', 1, 1, '2023-09-10 12:00:00', '2023-09-10 12:00:00', 'System', 'System'),
    (62, 2, 'Week 38 2023', '2023-09-17 12:00:00', '2023-09-23 11:59:59', 1, 1, '2023-09-17 12:00:00', '2023-09-17 12:00:00', 'System', 'System'),
    (63, 2, 'Week 39 2023', '2023-09-24 12:00:00', '2023-09-30 11:59:59', 1, 1, '2023-09-24 12:00:00', '2023-09-24 12:00:00', 'System', 'System'),
    (64, 2, 'Week 40 2023', '2023-10-01 12:00:00', '2023-10-07 11:59:59', 1, 1, '2023-10-01 12:00:00', '2023-10-01 12:00:00', 'System', 'System'),
    (65, 2, 'Week 41 2023', '2023-10-08 12:00:00', '2023-10-14 11:59:59', 1, 1, '2023-10-08 12:00:00', '2023-10-08 12:00:00', 'System', 'System'),
    (66, 2, 'Week 42 2023', '2023-10-15 12:00:00', '2023-10-21 11:59:59', 1, 1, '2023-10-15 12:00:00', '2023-10-15 12:00:00', 'System', 'System'),
    (67, 2, 'Week 43 2023', '2023-10-22 12:00:00', '2023-10-28 11:59:59', 1, 1, '2023-10-22 12:00:00', '2023-10-22 12:00:00', 'System', 'System'),
    (68, 2, 'Week 44 2023', '2023-10-29 12:00:00', '2023-11-04 11:59:59', 1, 1, '2023-10-29 12:00:00', '2023-10-29 12:00:00', 'System', 'System'),
    (69, 2, 'Week 45 2023', '2023-11-05 12:00:00', '2023-11-11 11:59:59', 1, 1, '2023-11-05 12:00:00', '2023-11-05 12:00:00', 'System', 'System'),
    (70, 2, 'Week 46 2023', '2023-11-12 12:00:00', '2023-11-18 11:59:59', 1, 1, '2023-11-12 12:00:00', '2023-11-12 12:00:00', 'System', 'System'),
    (71, 2, 'Week 47 2023', '2023-11-19 12:00:00', '2023-11-25 11:59:59', 1, 1, '2023-11-19 12:00:00', '2023-11-19 12:00:00', 'System', 'System'),
    (72, 2, 'Week 48 2023', '2023-11-26 12:00:00', '2023-12-02 11:59:59', 1, 1, '2023-11-26 12:00:00', '2023-11-26 12:00:00', 'System', 'System'),
    (73, 2, 'Week 49 2023', '2023-12-03 12:00:00', '2023-12-09 11:59:59', 1, 1, '2023-12-03 12:00:00', '2023-12-03 12:00:00', 'System', 'System'),
    (74, 2, 'Week 50 2023', '2023-12-10 12:00:00', '2023-12-16 11:59:59', 1, 1, '2023-12-10 12:00:00', '2023-12-10 12:00:00', 'System', 'System'),
    (75, 2, 'Week 51 2023', '2023-12-17 12:00:00', '2023-12-23 11:59:59', 1, 1, '2023-12-17 12:00:00', '2023-12-17 12:00:00', 'System', 'System'),
    (76, 2, 'Week 52 2023', '2023-12-24 12:00:00', '2023-12-30 11:59:59', 1, 1, '2023-12-24 12:00:00', '2023-12-24 12:00:00', 'System', 'System'),
    (77, 2, 'Week 1 2024', '2023-12-31 12:00:00', '2024-01-06 11:59:59', 1, 1, '2023-12-31 12:00:00', '2023-12-31 12:00:00', 'System', 'System'),
    (78, 2, 'Week 2 2024', '2024-01-07 12:00:00', '2024-01-13 11:59:59', 1, 1, '2024-01-07 12:00:00', '2024-01-07 12:00:00', 'System', 'System'),
    (79, 2, 'Week 3 2024', '2024-01-14 12:00:00', '2024-01-20 11:59:59', 1, 1, '2024-01-14 12:00:00', '2024-01-14 12:00:00', 'System', 'System'),
    (80, 2, 'Week 4 2024', '2024-01-21 12:00:00', '2024-01-27 11:59:59', 1, 1, '2024-01-21 12:00:00', '2024-01-21 12:00:00', 'System', 'System'),
    (81, 2, 'Week 5 2024', '2024-01-28 12:00:00', '2024-02-03 11:59:59', 1, 1, '2024-01-28 12:00:00', '2024-01-28 12:00:00', 'System', 'System'),
    (82, 2, 'Week 6 2024', '2024-02-04 12:00:00', '2024-02-10 11:59:59', 1, 1, '2024-02-04 12:00:00', '2024-02-04 12:00:00', 'System', 'System'),
    (83, 2, 'Week 7 2024', '2024-02-11 12:00:00', '2024-02-17 11:59:59', 1, 1, '2024-02-11 12:00:00', '2024-02-11 12:00:00', 'System', 'System'),
    (84, 2, 'Week 8 2024', '2024-02-18 12:00:00', '2024-02-24 11:59:59', 1, 1, '2024-02-18 12:00:00', '2024-02-18 12:00:00', 'System', 'System'),
    (85, 2, 'Week 9 2024', '2024-02-25 12:00:00', '2024-03-02 11:59:59', 1, 1, '2024-02-25 12:00:00', '2024-02-25 12:00:00', 'System', 'System'),
    (86, 2, 'Week 10 2024', '2024-03-03 12:00:00', '2024-03-09 11:59:59', 1, 1, '2024-03-03 12:00:00', '2024-03-03 12:00:00', 'System', 'System'),
    (87, 2, 'Week 11 2024', '2024-03-10 12:00:00', '2024-03-16 11:59:59', 1, 1, '2024-03-10 12:00:00', '2024-03-10 12:00:00', 'System', 'System'),
    (88, 2, 'Week 12 2024', '2024-03-17 12:00:00', '2024-03-23 11:59:59', 1, 1, '2024-03-17 12:00:00', '2024-03-17 12:00:00', 'System', 'System'),
    (89, 2, 'Week 13 2024', '2024-03-24 12:00:00', '2024-03-30 11:59:59', 1, 1, '2024-03-24 12:00:00', '2024-03-24 12:00:00', 'System', 'System'),
    (90, 2, 'Week 14 2024', '2024-03-31 12:00:00', '2024-04-06 11:59:59', 1, 1, '2024-03-31 12:00:00', '2024-03-31 12:00:00', 'System', 'System'),
    (91, 2, 'Week 15 2024', '2024-04-07 12:00:00', '2024-04-13 11:59:59', 1, 1, '2024-04-07 12:00:00', '2024-04-07 12:00:00', 'System', 'System'),
    (92, 2, 'Week 16 2024', '2024-04-14 12:00:00', '2024-04-20 11:59:59', 1, 1, '2024-04-14 12:00:00', '2024-04-14 12:00:00', 'System', 'System'),
    (93, 2, 'Week 17 2024', '2024-04-21 12:00:00', '2024-04-27 11:59:59', 1, 1, '2024-04-21 12:00:00', '2024-04-21 12:00:00', 'System', 'System'),
    (94, 2, 'Week 18 2024', '2024-04-28 12:00:00', '2024-05-04 11:59:59', 1, 1, '2024-04-28 12:00:00', '2024-04-28 12:00:00', 'System', 'System'),
    (95, 2, 'Week 19 2024', '2024-05-05 12:00:00', '2024-05-11 11:59:59', 1, 1, '2024-05-05 12:00:00', '2024-05-05 12:00:00', 'System', 'System'),
    (96, 2, 'Week 20 2024', '2024-05-12 12:00:00', '2024-05-18 11:59:59', 1, 1, '2024-05-12 12:00:00', '2024-05-12 12:00:00', 'System', 'System'),
    (97, 2, 'Week 21 2024', '2024-05-19 12:00:00', '2024-05-25 11:59:59', 1, 1, '2024-05-19 12:00:00', '2024-05-19 12:00:00', 'System', 'System'),
    (98, 2, 'Week 22 2024', '2024-05-26 12:00:00', '2024-06-01 11:59:59', 1, 1, '2024-05-26 12:00:00', '2024-05-26 12:00:00', 'System', 'System'),
    (99, 2, 'Week 23 2024', '2024-06-02 12:00:00', '2024-06-08 11:59:59', 1, 1, '2024-06-02 12:00:00', '2024-06-02 12:00:00', 'System', 'System'),
    (100, 2, 'Week 24 2024', '2024-06-09 12:00:00', '2024-06-15 11:59:59', 1, 1, '2024-06-09 12:00:00', '2024-06-09 12:00:00', 'System', 'System'),
    (101, 2, 'Week 25 2024', '2024-06-16 12:00:00', '2024-06-22 11:59:59', 1, 1, '2024-06-16 12:00:00', '2024-06-16 12:00:00', 'System', 'System'),
    (102, 2, 'Week 26 2024', '2024-06-23 12:00:00', '2024-06-29 11:59:59', 1, 1, '2024-06-23 12:00:00', '2024-06-23 12:00:00', 'System', 'System'),
    (103, 2, 'Week 27 2024', '2024-06-30 12:00:00', '2024-07-06 11:59:59', 1, 1, '2024-06-30 12:00:00', '2024-06-30 12:00:00', 'System', 'System'),
    (104, 2, 'Week 28 2024', '2024-07-07 12:00:00', '2024-07-13 11:59:59', 1, 1, '2024-07-07 12:00:00', '2024-07-07 12:00:00', 'System', 'System'),
    (105, 2, 'Week 29 2024', '2024-07-14 12:00:00', '2024-07-20 11:59:59', 1, 1, '2024-07-14 12:00:00', '2024-07-14 12:00:00', 'System', 'System'),
    (106, 2, 'Week 30 2024', '2024-07-21 12:00:00', '2024-07-27 11:59:59', 1, 1, '2024-07-21 12:00:00', '2024-07-21 12:00:00', 'System', 'System'),
    (107, 2, 'Week 31 2024', '2024-07-28 12:00:00', '2024-08-03 11:59:59', 1, 1, '2024-07-28 12:00:00', '2024-07-28 12:00:00', 'System', 'System'),
    (108, 2, 'Week 32 2024', '2024-08-04 12:00:00', '2024-08-10 11:59:59', 1, 1, '2024-08-04 12:00:00', '2024-08-04 12:00:00', 'System', 'System'),
    (109, 2, 'Week 33 2024', '2024-08-11 12:00:00', '2024-08-17 11:59:59', 1, 1, '2024-08-11 12:00:00', '2024-08-11 12:00:00', 'System', 'System'),
    (110, 2, 'Week 34 2024', '2024-08-18 12:00:00', '2024-08-24 11:59:59', 1, 1, '2024-08-18 12:00:00', '2024-08-18 12:00:00', 'System', 'System'),
    (111, 2, 'Week 35 2024', '2024-08-25 12:00:00', '2024-08-31 11:59:59', 1, 1, '2024-08-25 12:00:00', '2024-08-25 12:00:00', 'System', 'System'),
    (112, 2, 'Week 36 2024', '2024-09-01 12:00:00', '2024-09-07 11:59:59', 1, 1, '2024-09-01 12:00:00', '2024-09-01 12:00:00', 'System', 'System'),
    (113, 2, 'Week 37 2024', '2024-09-08 12:00:00', '2024-09-14 11:59:59', 1, 1, '2024-09-08 12:00:00', '2024-09-08 12:00:00', 'System', 'System'),
    (114, 2, 'Week 38 2024', '2024-09-15 12:00:00', '2024-09-21 11:59:59', 1, 1, '2024-09-15 12:00:00', '2024-09-15 12:00:00', 'System', 'System'),
    (115, 2, 'Week 39 2024', '2024-09-22 12:00:00', '2024-09-28 11:59:59', 1, 1, '2024-09-22 12:00:00', '2024-09-22 12:00:00', 'System', 'System'),
    (116, 2, 'Week 40 2024', '2024-09-29 12:00:00', '2024-10-05 11:59:59', 1, 1, '2024-09-29 12:00:00', '2024-09-29 12:00:00', 'System', 'System'),
    (117, 2, 'Week 41 2024', '2024-10-06 12:00:00', '2024-10-12 11:59:59', 1, 1, '2024-10-06 12:00:00', '2024-10-06 12:00:00', 'System', 'System'),
    (118, 2, 'Week 42 2024', '2024-10-13 12:00:00', '2024-10-19 11:59:59', 1, 1, '2024-10-13 12:00:00', '2024-10-13 12:00:00', 'System', 'System'),
    (119, 2, 'Week 43 2024', '2024-10-20 12:00:00', '2024-10-26 11:59:59', 1, 1, '2024-10-20 12:00:00', '2024-10-20 12:00:00', 'System', 'System'),
    (120, 2, 'Week 44 2024', '2024-10-27 12:00:00', '2024-11-02 11:59:59', 1, 1, '2024-10-27 12:00:00', '2024-10-27 12:00:00', 'System', 'System'),
    (121, 2, 'Week 45 2024', '2024-11-03 12:00:00', '2024-11-09 11:59:59', 1, 1, '2024-11-03 12:00:00', '2024-11-03 12:00:00', 'System', 'System'),
    (122, 2, 'Week 46 2024', '2024-11-10 12:00:00', '2024-11-16 11:59:59', 1, 1, '2024-11-10 12:00:00', '2024-11-10 12:00:00', 'System', 'System'),
    (123, 2, 'Week 47 2024', '2024-11-17 12:00:00', '2024-11-23 11:59:59', 1, 1, '2024-11-17 12:00:00', '2024-11-17 12:00:00', 'System', 'System'),
    (124, 2, 'Week 48 2024', '2024-11-24 12:00:00', '2024-11-30 11:59:59', 1, 1, '2024-11-24 12:00:00', '2024-11-24 12:00:00', 'System', 'System'),
    (125, 2, 'Week 49 2024', '2024-12-01 12:00:00', '2024-12-07 11:59:59', 1, 1, '2024-12-01 12:00:00', '2024-12-01 12:00:00', 'System', 'System'),
    (126, 2, 'Week 50 2024', '2024-12-08 12:00:00', '2024-12-14 11:59:59', 1, 1, '2024-12-08 12:00:00', '2024-12-08 12:00:00', 'System', 'System'),
    (127, 2, 'Week 51 2024', '2024-12-15 12:00:00', '2024-12-21 11:59:59', 1, 1, '2024-12-15 12:00:00', '2024-12-15 12:00:00', 'System', 'System'),
    (128, 2, 'Week 52 2024', '2024-12-22 12:00:00', '2024-12-28 11:59:59', 1, 1, '2024-12-22 12:00:00', '2024-12-22 12:00:00', 'System', 'System');

-- Insert 1 row into the Customer table
INSERT INTO Customer
    (customer_id, company_id, customer_type_id, customer_status_id, customer_sub_status_id, enroller_id, sponsor_id, binary_placement_id)
VALUES
    (1, 1, 1, 1, 1, 1, 1, 1);


-- Insert 1 row into the order table
-- INSERT INTO Orders
-- SET order_id = 1,
--     company_id = 1,
--     customer_id = 1,
--     order_status_id = 1,
--     order_date = '2023-01-01 12:00:00',
--     currency_code = 'USD',
--     warehouse_id = 1,
--     ship_method_id = 1,
--     order_type_id = 1,
--     price_type_id = 1,
--     first_name = 'John',
--     middle_name = 'A',
--     last_name = 'Doe',
--     name_suffix = 'Jr',
--     company = 'Rust Engine',
--     address1 = '123 Main St',
--     address2 = 'Suite 100',
--     address3 = 'Building 5',
--     city = 'Anytown',
--     `state` = 'TX',
--     zip = '12345',
--     country = 'USA',
--     county = 'Any County',
--     email = 'john.doe@example.com',
--     phone = '123-456-7890',
--     notes = 'Test Order',
--     total = 100.00,
--     sub_total = 90.00,
--     tax_total = 5.0,
--     shipping_total = 5.0,
--     discount_total = 10.0,
--     discount_percent = 10.0,
--     weight_total = 2.5,
--     business_volume_total = 50.00,
--     commissionable_volume_total = 50.00,
--     other1_total = 0.0,
--     other2_total = 0.0,
--     other3_total = 0.0,
--     other4_total = 0.0,
--     other5_total = 0.0,
--     other6_total = 0.0,
--     other7_total = 0.0,
--     other8_total = 0.0,
--     other9_total = 0.0,
--     other10_total = 0.0,
--     shipping_tax = 1.5,
--     order_tax = 1.5,
--     fed_tax_total = 0.5,
--     state_tax_total = 1.0,
--     fed_shipping_tax = 0.25,
--     state_shipping_tax = 0.25,
--     city_shipping_tax = 0.10,
--     city_local_shipping_tax = 0.10,
--     county_shipping_tax = 0.05,
--     county_local_shipping_tax = 0.05,
--     other11 = 'Extra Note 1',
--     other12 = 'Extra Note 2',
--     other13 = 'Extra Note 3',
--     other14 = 'Extra Note 4',
--     other15 = 'Extra Note 5',
--     other16 = 'Extra Note 6',
--     other17 = 'Extra Note 7',
--     other18 = 'Extra Note 8',
--     other19 = 'Extra Note 9',
--     other20 = 'Extra Note 10',
--     is_commissionable = TRUE,
--     auto_order_id = 2,
--     return_order_id = 3,
--     replacement_order_id = 4,
--     parent_order_id = 5,
--     decline_count = 0,
--     transfer_to_customer_id = 6,
--     party_id = 7,
--     shipped_date = '2023-01-02 10:00:00',
--     created_date = '2023-01-01 09:00:00',
--     locked_date = '2023-01-01 11:00:00',
--     modified_date = '2023-01-02 08:00:00',
--     created_by = 'admin',
--     modified_by = 'user',
--     tax_integration_calculate = TRUE,
--     tax_integration_commit = TRUE,
--     handling_fee = 2.0,
--     pickup_name = 'Pickup Location',
--     total_taxable = 85.00,
--     order_sub_status_id = 1,
--     referral_id = 123;

-- -- Insert 2 rows into the order detail table 
-- INSERT INTO OrderDetail
-- SET order_id = 1,
--     order_line = 1,
--     order_detail_id = 1,
--     parent_order_detail_id = 0,
--     item_id = 1,
--     item_code = 'SKU-1',
--     item_description = 'Product 1',
--     quantity = 1,
--     price_each = 10.00,
--     price_total = 10.00,
--     tax = 0.00,
--     weight_each = 1.00,
--     `weight` = 1.00,
--     business_volume_each = 10.00,
--     business_volume = 10.00,
--     commissionable_volume_each = 1.00,
--     commissionable_volume = 1.00,
--     other1_each = 1.00,
--     other1 = 1.00,
--     other2_each = 1.00,
--     other2 = 1.00,
--     other3_each = 1.00,
--     other3 = 1.00,
--     other4_each = 1.00,
--     other4 = 1.00,
--     other5_each = 1.00,
--     other5 = 1.00,
--     original_taxable_each = 1.00,
--     original_business_volume_each = 1.00,
--     original_commissionable_volume_each = 1.00,
--     other6_each = 1.00,
--     other6 = 1.00,
--     other7_each = 1.00,
--     other7 = 1.00,
--     other8_each = 1.00,
--     other8 = 1.00,
--     other9_each = 1.00,
--     other9 = 1.00,
--     other10_each = 1.00,
--     other10 = 1.00,
--     parent_item_id = 1,
--     taxable = 1.00,
--     fed_tax = 1.00,
--     state_tax = 1.00,
--     city_tax = 1.00,
--     city_local_tax = 1.00,
--     county_tax = 1.00,
--     county_local_tax = 1.00,
--     manual_tax = 1.00,
--     is_state_tax_override = 1,
--     reference1 = 'Ref1';
