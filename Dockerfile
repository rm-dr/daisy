FROM ubuntu AS node
RUN apt-get update
RUN apt-get install cargo npm -y
COPY ./site ./site
RUN cd /site && npm install

FROM ubuntu
RUN apt-get update
RUN apt-get install nginx -y
COPY --from=node /site /var/www/html
COPY ./pkg /var/www/html/pkg
COPY default /etc/nginx/sites-enabled/default
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]