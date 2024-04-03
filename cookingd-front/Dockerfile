FROM node:latest as build-stage
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY ./ .
RUN ls -la
RUN pwd
RUN npm run build

FROM nginx as production-stage
RUN mkdir /app
RUN ls -la /etc/nginx/
COPY --from=build-stage /app/cert.pem /etc/nginx/
COPY --from=build-stage /app/cert.key /etc/nginx/
RUN ls -la /etc/nginx/
COPY --from=build-stage /app/dist /app
COPY nginx.conf /etc/nginx/nginx.conf
ENTRYPOINT nginx -g 'daemon off;'