## overview
all authetntication pages should be in `/auth/*` path

## login flow
- user boots up, if not authenticated, app loads `/auth/login'`
- if any user has signed in, shows signup form form
  - form has username field and password fields (with confirmation)
  - when user submit, user is created.
  - note there is no need to use email, as device will generate unique code for that user.
  - redirect to login page
- login page shouls shows the following components
  - user selection (avatar likes election, displayed like Netflix user selection)
  - if user is selected
    - Display name shows
    - password field shows
    - submit button shows
  - if auth not valid, shows error message
  - is auth is valid, redirect to `/notes/` page

# asset
- [login page](src-tauri/src/ui/stitch/simple_user_login)

# tasklists
- [ ] create ui componenets in `src/lib/components/auth/` (with storybook)
- [ ] create auth backend in tauri
- [ ] create page from used components and plug the tauir commands
