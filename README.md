# DiscOverlay

DiscOverlay is an app that provides native overlay windows displaying Discord info. It shows information like who is currently in the user's connected voice channel and the voice states of those users. A message overlay feature, similar to Discord's StreamKit, is also being developed to enhance its capabilities.

There are several options for customization, such as changing which users are displayed, what information about them is shown and what other channel information is displayed. There is also a custom CSS option for those who want to further customize the appearance of the overlay.

[![wakatime](https://wakatime.com/badge/user/25be8ed5-7461-4fcf-93f7-0d88a7692cca/project/10d4eb9c-9912-4f2e-b38e-d507e6618a1f.svg)](https://wakatime.com/badge/user/25be8ed5-7461-4fcf-93f7-0d88a7692cca/project/10d4eb9c-9912-4f2e-b38e-d507e6618a1f)

---

[![Discord Badge](https://raw.githubusercontent.com/intergrav/devins-badges/v2/assets/cozy/social/discord-singular_64h.png)](https://s.deftu.dev/discord)
[![Ko-Fi Badge](https://raw.githubusercontent.com/intergrav/devins-badges/v2/assets/cozy/donate/kofi-singular_64h.png)](https://s.deftu.dev/kofi)

---

## Setup

### Creating your Discord developer application

Navigate to the Discord developer portal create a new developer application

![New Application](./static/new-application.png)

Create an application. This can be named whatever you like and can be within whatever team you would like. If you don’t know what the teams mean, just leave it on “Personal”

![Create Application](./static/create-application.png)

### Setting up your client

Once you’re taken to your new developer app’s page, navigate to the “OAuth2” tab

![Navigate to OAuth2 Tab](./static/navigate-oauth2.png)

Copy your Client ID and (reset and) Client Secret, then input both into the fields below

![Copy Client ID and Secret](./static/copy-client-id-and-secret.png)

Finally, click “Add Redirect” and input http://localhost:3053/redirect into the redirect input, then save your changes

![Add Redirect](./static/add-oauth2-redirect.png)

---

[![BisectHosting](https://www.bisecthosting.com/partners/custom-banners/8fb6621b-811a-473b-9087-c8c42b50e74c.png)](https://s.deftu.dev/bisect)

---

**This project is licensed under [LGPL-3.0][lgpl]**\
**&copy; 2024 Deftu**

[lgpl]: https://www.gnu.org/licenses/lgpl-3.0.en.html
