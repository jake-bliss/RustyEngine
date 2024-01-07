up-db:
	docker run --name rustenginemysql -e MYSQL_ROOT_PASSWORD=thisisit -p 3306:3306 -v rustengine-mysql-data:/var/lib/mysql -d mysql

down-db:
	docker stop rustenginemysql
	docker rm rustenginemysql

connect-db:
	mysql -u root -p -h 127.0.0.1 --protocol=TCP -P 3306

createtables-db:
	mysql -u root -p -h 127.0.0.1 --protocol=TCP -P 3306 < create-script.sql

droptables-db:
	mysql -u root -p -h 127.0.0.1 --protocol=TCP -P 3306 < drop-script.sql

createtables-cloud-db:
	mysql -u staging_db_user_developer -p -h 35.225.39.87 --protocol=TCP -P 3306 < create-script.sql

droptables-cloud-db:
	mysql -u staging_db_user_developer -p -h 35.225.39.87 --protocol=TCP -P 3306 < drop-script.sql

docker-build:
	docker build -t rustengine .

docker-run:
	docker run --name rustengine -p 3030:3030 -d rustengineW