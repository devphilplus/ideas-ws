FROM swaggerapi/swagger-ui:latest
LABEL maintainer="Ferd Tomale<beowulf1416@gmail.com>"

COPY ./auth.json /usr/share/nginx/html/definitions/auth.json
COPY ./user.json /usr/share/nginx/html/definitions/user.json
COPY ./client.json /usr/share/nginx/html/definitions/client.json

EXPOSE 8080/tcp
EXPOSE 8080/udp

ENV SWAGGER_JSON_URL=/definitions/auth.json