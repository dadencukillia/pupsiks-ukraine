# 🥺 Association Pupsiks of Ukraine
<img src="https://raw.githubusercontent.com/dadencukillia/pupsiks-ukraine/refs/heads/main/docs/assets/logo.png" alt="Pupsiks logo" align="left" height="128">

Association Pupsiks of Ukraine (**APU**) — A fun project that aims to bring together all the kind people of Ukraine. Here you can register for a certificate to confirm your status as one of the kindest people. Don't forget to share the link to this certificate with your friends!

![SvelteKit](https://img.shields.io/badge/SvelteKit-%23f1413d.svg?logo=svelte&logoColor=white)
![Tailwind CSS](https://img.shields.io/badge/Tailwind%20CSS-%2338B2AC.svg?logo=tailwind-css&logoColor=white)
![Actix](https://img.shields.io/badge/Actix-%230A141C.svg?logo=actix&logoColor=white)
![SeaORM](https://img.shields.io/badge/SeaORM-%232171DE.svg?logo=laravelhorizon&logoColor=white)
![Redis](https://img.shields.io/badge/Redis-%23DD0031.svg?logo=redis&logoColor=white)
![Postgres](https://img.shields.io/badge/Postgres-%23316192.svg?logo=postgresql&logoColor=white)

## Features
- Creating a certificate with a code sent to email
- Sharing a certificate via Telegram
- Deleting a certificate by sending a code to email
- Looking for a certificate by serial number

## Screenshots
<div style="display: flex; flex-direction: row; gap: 10px;">
  <img height="300" alt="Homepage" src="https://github.com/user-attachments/assets/584ade17-3bdf-4624-a53c-70fa432ac4c4" />
  <img height="300" alt="Certificate registration" src="https://github.com/user-attachments/assets/25be3960-c1bf-48af-80f5-7a23c6579ee7" />
  <img height="300" alt="Certificate viewing" src="https://github.com/user-attachments/assets/f488b863-b7c3-432d-a081-73af347307fd" />
</div>


# 🎯 Goals

## My goals in the project
As I said earlier, this is a **fun project**. My goal was to use common practices (MVC, CI/CD, ORM) and technologies (SvelteKit, TailwindCSS, Actix, Redis) that I hadn't used before. Overall, this project is educational. I **don't plan to develop it further** in the future, but it can serve as a good project in my portfolio.

## Practical value
This project is one of the few written on Actix (Rust). And although SvelteKit is more widespread, it is still rare to find projects using it on the internet. So this project **can serve as an example or inspiration** for your projects if you write them on Actix or SvelteKit.

I also spent quite a bit of time setting up CI/CD and getting Certbot to work with Nginx. So **feel free to use the practices and code snippets** from this repository.


# 🪚 Development
This project uses Docker and Docker Compose to run. It also includes Backend, Frontend, and a messaging service. Each of them is divided into corresponding folders.

## Run on HTTP protocol
You need to clone the repository with this command:
```bash
git clone https://github.com/dadencukillia/pupsiks-ukraine.git
```

Rename the `.env.example` file to `.env` and fill it in with your details (in your case, the `SSL_OWNER_EMAIL` and `TEST_EMAIL` fields are not mandatory).

Also, if you don't have a domain, you can specify `127.0.0.1` in the `SERVER_NAME` field for access from your device or WAN IP addresses for public access. This is important because Nginx is configured to redirect any users to the specified address if it is not located there.

To start the website, type the following commands (you may need sudo)::
```bash
make build_production
make production
```

## Run on HTTPS protocol
You need to clone the repository with this command:
```bash
git clone https://github.com/dadencukillia/pupsiks-ukraine.git
```

Rename the `.env.example` file to `.env` and fill it in with your details (in your case, the `TEST_EMAIL` field is not mandatory).

In the `SERVER_NAME` field, specify your domain without `https://` and without `http://` fragments. Paths are also not allowed, i.e. you must not use the `/` character at all.

Once you have specified everything, make sure that the domain points to the IP address of the device on which the website will be launched.

If you have completed all the tasks, it is time to obtain your first SSL certificates (this must be done with Docker services turned off). To obtain the certificates, enter the command (sudo may be required):
```bash
make first_ssl_certs
```

If everything goes smoothly, you now have SSL certificates! You can now start the main website services with the following commands (you may need sudo):
```bash
make build_production_https
make production_https
```

The website should now be operational under the domain you specified! But keep in mind that you need to regularly update SSL certificates because they are temporary. You can safely check certificates for updates once a week (at the request of the verifier, this should be done at random times), and if the utility detects the need to update certificates, it will update them automatically. To run the certificate verification and update utility, enter the command (sudo may be required):
```bash
make renew_ssl_certs
```

## Run in development mode
If you want to update the code and view the changes, you will need “development” mode. It opens the site at `http://127.0.0.1`, and when you update the Frontend code, you just need to refresh the page to view the changes. Unfortunately, this does not work on Rust, and you need to restart the command each time.

So, you need to clone the repository, rename the `.env.example` file to `.env`, and fill in your details (all fields except `SMTP_SERVER`, `SMTP_PORT`, `SMTP_SENDER`, `SMTP_USERNAME`, and `SMTP_PASSWORD` are not used in this mode, so you don't need to fill them in).

Once you have filled in all the fields, you can start development mode with the command (you may need sudo):
```bash
make dev
```

## Run testing
After making changes to the code, you may want to run Backend testing (if you have improved the Backend, make sure you have added tests to it). To do this, you will need an additional field in `.env` called `TEST_EMAIL` (in reality, tests no longer send emails, so you can assign any value to this field).

To run the tests, enter the command (you may need sudo):
```bash
make test
```
