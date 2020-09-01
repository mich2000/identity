# Build stage
FROM node:14-alpine as build-stage

WORKDIR /app

COPY ./identity_react/package*.json /app/

RUN npm install

COPY ./identity_react/ /app/

RUN npm run build


# Final build state
FROM nginx:alpine

EXPOSE 80

COPY --from=build-stage /app/build/ /usr/share/nginx/html

COPY --from=build-stage /nginx.conf /etc/nginx/conf.d/default.conf