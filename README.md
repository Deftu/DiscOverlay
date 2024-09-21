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

DiscOverlay uses Discord's RPC API to interact with your client for all of the information it needs to display. Unfortunately, as of now, Discord does not allow for global applications to use the RPC API, so you will need to create a new personal application on the Discord Developer Portal.

![New Application screenshot](./.github/new_application.png)

### Getting your client ID

Discord uses this for determining which application to interact with. You can find this on the general information page of your application.

![Client ID screenshot](./.github/copy_client_id.png)

### Setting your redirect URI

The app redirects you to this URI after you authorize it. You can set this in the OAuth2 tab of your application.

![Redirect URI screenshot](./.github/redirect_url.png)

---

[![BisectHosting](https://www.bisecthosting.com/partners/custom-banners/8fb6621b-811a-473b-9087-c8c42b50e74c.png)](https://s.deftu.dev/bisect)

---

**This project is licensed under [LGPL-3.0][lgpl]**\
**&copy; 2024 Deftu**

[lgpl]: https://www.gnu.org/licenses/lgpl-3.0.en.html
