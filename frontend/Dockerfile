FROM node:12-alpine as build

COPY . .
RUN npm i && npm run build

FROM nginx:1.17-alpine
COPY --from=build /dist /usr/share/nginx/html