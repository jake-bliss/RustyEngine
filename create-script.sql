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
    customer_id INT PRIMARY KEY,
    company_id INT,
    customer_type_id INT,
    customer_status_id INT,
    customer_sub_status_id INT,
    enroller_id INT,
    sponsor_id INT,
    binary_placement_id INT,
    FOREIGN KEY (company_id) REFERENCES Company (company_id),
    FOREIGN KEY (enroller_id) REFERENCES Customer (customer_id),
    FOREIGN KEY (sponsor_id) REFERENCES Customer (customer_id),
    FOREIGN KEY (binary_placement_id) REFERENCES Customer (customer_id)
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
    shipped_date DATETIME,
    created_date DATETIME,
    locked_date DATETIME,
    modified_date DATETIME,
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

-- Insert 12 rows into period table

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
    (12, 1, 'December 2023', '2023-12-01 12:00:00', '2023-12-31 11:59:59', 1, 1, '2023-12-01 12:00:00', '2023-12-01 12:00:00', 'System', 'System');

-- Insert 1 row into the Customer table
INSERT INTO Customer
    (customer_id, company_id, customer_type_id, customer_status_id, customer_sub_status_id, enroller_id, sponsor_id, binary_placement_id)
VALUES
    (1, 1, 1, 1, 1, 1, 1, 1);


-- Insert 1 row into the order table
INSERT INTO Orders
SET order_id = 1,
    company_id = 1,
    customer_id = 1,
    order_status_id = 1,
    order_date = '2023-01-01 12:00:00',
    currency_code = 'USD',
    warehouse_id = 1,
    ship_method_id = 1,
    order_type_id = 1,
    price_type_id = 1,
    first_name = 'John',
    middle_name = 'A',
    last_name = 'Doe',
    name_suffix = 'Jr',
    company = 'Rust Engine',
    address1 = '123 Main St',
    address2 = 'Suite 100',
    address3 = 'Building 5',
    city = 'Anytown',
    `state` = 'TX',
    zip = '12345',
    country = 'USA',
    county = 'Any County',
    email = 'john.doe@example.com',
    phone = '123-456-7890',
    notes = 'Test Order',
    total = 100.00,
    sub_total = 90.00,
    tax_total = 5.0,
    shipping_total = 5.0,
    discount_total = 10.0,
    discount_percent = 10.0,
    weight_total = 2.5,
    business_volume_total = 50.00,
    commissionable_volume_total = 50.00,
    other1_total = 0.0,
    other2_total = 0.0,
    other3_total = 0.0,
    other4_total = 0.0,
    other5_total = 0.0,
    other6_total = 0.0,
    other7_total = 0.0,
    other8_total = 0.0,
    other9_total = 0.0,
    other10_total = 0.0,
    shipping_tax = 1.5,
    order_tax = 1.5,
    fed_tax_total = 0.5,
    state_tax_total = 1.0,
    fed_shipping_tax = 0.25,
    state_shipping_tax = 0.25,
    city_shipping_tax = 0.10,
    city_local_shipping_tax = 0.10,
    county_shipping_tax = 0.05,
    county_local_shipping_tax = 0.05,
    other11 = 'Extra Note 1',
    other12 = 'Extra Note 2',
    other13 = 'Extra Note 3',
    other14 = 'Extra Note 4',
    other15 = 'Extra Note 5',
    other16 = 'Extra Note 6',
    other17 = 'Extra Note 7',
    other18 = 'Extra Note 8',
    other19 = 'Extra Note 9',
    other20 = 'Extra Note 10',
    is_commissionable = TRUE,
    auto_order_id = 2,
    return_order_id = 3,
    replacement_order_id = 4,
    parent_order_id = 5,
    decline_count = 0,
    transfer_to_customer_id = 6,
    party_id = 7,
    shipped_date = '2023-01-02 10:00:00',
    created_date = '2023-01-01 09:00:00',
    locked_date = '2023-01-01 11:00:00',
    modified_date = '2023-01-02 08:00:00',
    created_by = 'admin',
    modified_by = 'user',
    tax_integration_calculate = TRUE,
    tax_integration_commit = TRUE,
    handling_fee = 2.0,
    pickup_name = 'Pickup Location',
    total_taxable = 85.00,
    order_sub_status_id = 1,
    referral_id = 123;

-- Insert 2 rows into the order detail table 
INSERT INTO OrderDetail
SET order_id = 1,
    order_line = 1,
    order_detail_id = 1,
    parent_order_detail_id = 0,
    item_id = 1,
    item_code = 'SKU-1',
    item_description = 'Product 1',
    quantity = 1,
    price_each = 10.00,
    price_total = 10.00,
    tax = 0.00,
    weight_each = 1.00,
    `weight` = 1.00,
    business_volume_each = 10.00,
    business_volume = 10.00,
    commissionable_volume_each = 1.00,
    commissionable_volume = 1.00,
    other1_each = 1.00,
    other1 = 1.00,
    other2_each = 1.00,
    other2 = 1.00,
    other3_each = 1.00,
    other3 = 1.00,
    other4_each = 1.00,
    other4 = 1.00,
    other5_each = 1.00,
    other5 = 1.00,
    original_taxable_each = 1.00,
    original_business_volume_each = 1.00,
    original_commissionable_volume_each = 1.00,
    other6_each = 1.00,
    other6 = 1.00,
    other7_each = 1.00,
    other7 = 1.00,
    other8_each = 1.00,
    other8 = 1.00,
    other9_each = 1.00,
    other9 = 1.00,
    other10_each = 1.00,
    other10 = 1.00,
    parent_item_id = 1,
    taxable = 1.00,
    fed_tax = 1.00,
    state_tax = 1.00,
    city_tax = 1.00,
    city_local_tax = 1.00,
    county_tax = 1.00,
    county_local_tax = 1.00,
    manual_tax = 1.00,
    is_state_tax_override = 1,
    reference1 = 'Ref1';
