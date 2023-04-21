# Stage 1 - Build React App
FROM node:18-alpine as build

WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json .
RUN npm ci

COPY . ./
RUN npm run build

# Stage 2 - Serve the built app with nginx
# Pin the minor version
FROM nginx:1.23-alpine


# Replaced the external nginx.conf with this embeded one
# COPY nginx.conf /etc/nginx/conf.d/default.conf
RUN echo -e "server {\n\
    listen 8080;\n\
    server_name localhost;\n\
    root /usr/share/nginx/html;\n\
    index index.html;\n\
    location / {\n\
        try_files \$uri /index.html;\n\
    }\n\
}" > /etc/nginx/conf.d/default.conf

COPY --from=build /app/build /usr/share/nginx/html

EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
