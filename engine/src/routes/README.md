> [!tip]
> This is the beta version of edgeserver, see [edgeserver#stable](https://github.com/v3xlabs/edgeserver/tree/stable) for the stable version.

Hosting your own websites should be easy, and you should be able to do it.

| Feature          | v1 (stable) | v2 (beta)           |
| ---------------- | ----------- | ------------------- |
| Open Source      | ✓           | ✓                   |
| Teams            | x           | ✓                   |
| Domains          | ✓           | ✓                   |
| Wildcard Domains | x           | ✓                   |
| IPFS Pinning     | Partial     | ✓                   |
| ENS Updating     | Partial     | Not Implemented Yet |

<details>
    <summary>Examples</summary>
    You lost the game.
</details>

## Setting up Authentication

When you connect to edgeserver for the first time you will need to walk through the [bootstrapping](#tag/authentication/POST/auth/bootstrap) process (dont worry this is easy).
This should get you setup with an admin account on the instance.

If you have a user account on an instance see the [Team Keys](#tag/team/POST/team/{team_id}/keys) or [Site Keys](#tag/site/POST/site/{site_id}/keys) endpoints and menus (in the UI) to get yourself setup with a key.
