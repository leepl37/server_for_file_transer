FROM ubuntu:latest

WORKDIR /app
RUN apt-get update && apt-get upgrade -y && apt-get install -y
RUN apt-get install build-essential -y

RUN apt-get install libssl-dev -y
RUN apt-get install default-mysql-server -y
RUN apt-get install default-mysql-client -y
RUN apt-get install iputils-ping -y
RUN apt-get install telnet
RUN apt-get install f_file
RUN apt-get install -y sudo default-libmysqlclient-dev
RUN apt-get install -y sudo libmariadb-dev-compat libmariadb-dev
RUN apt-get install -y sudo vim
ADD target/release/thread_db_render_ref ./

ADD .env ./
COPY config ./config/

EXPOSE 8181

CMD ./thread_db_render_ref