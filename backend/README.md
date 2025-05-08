
# How to setup tls certificate

* Open terminal in root folder and create folder
```bash
mkdir .certs
```
```bash
cd .certs
```

* Then execute following command. Please ensure you have openssl. For windows you can find it in git's /user/bin folders 

```bash
openssl genrsa -out key.pem 2048
```

```bash
openssl req -new -key key.pem -out cert.csr
```

```bash
openssl x509 -req -days 365 -in cert.csr -signkey key.pem -out cert.pem
```

```bash
openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem
```

* Now you should have cert generated, so you can update .env file

```env
CERT_PATH=.certs/identity.pfx
CERT_PWD=1234
```