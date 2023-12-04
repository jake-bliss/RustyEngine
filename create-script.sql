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
    period_start_date DATE,
    period_end_date DATE,
    period_status_id INT,
    company_id INT,
    created_date DATE,
    modified_date DATE,
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
    is_active BOOLEAN,
    created_date DATE,
    modified_date DATE,
    created_by VARCHAR
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
    order_date DATE,
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
    shipped_date DATE,
    created_date DATE,
    locked_date DATE,
    modified_date DATE,
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