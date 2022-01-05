# étape de build
FROM node:16 as build-stage
WORKDIR /app
COPY package*.json ./
RUN export NODE_OPTIONS=--openssl-legacy-provider
RUN npm install
COPY . .
RUN npx browserslist@latest --update-db
RUN npm run build -- --mode development

# étape de production
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
