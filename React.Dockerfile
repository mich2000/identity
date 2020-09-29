# Build stage
FROM node:14-alpine as build-stage

WORKDIR /app

COPY ./identity_react/package*.json /app/

RUN npm install

COPY ./identity_react/ /app/

RUN npm run build --prod

# Final build state
FROM nginx:alpine

EXPOSE 80

RUN rm -rf /usr/share/nginx/html/*

COPY --from=build-stage /app/build/ /usr/share/nginx/html

COPY ./nginx.conf /etc/nginx/nginx.conf

ENTRYPOINT ["nginx", "-g", "daemon off;"]