Create DB User

mysql -u root -p

CREATE USER 'rustengine'@'localhost' IDENTIFIED BY 'rustpassword';
GRANT ALL PRIVILEGES ON *.* TO 'rustengine'@'localhost';
FLUSH PRIVILEGES;
EXIT;
