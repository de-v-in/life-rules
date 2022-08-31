# DEVIN NextJS Template

> Helping when start new frontend project

This is NextJS with React 18 template, which support auto format with eslint and intergrate `husky` for code checking before commit. This will help in smooth out project building.

## Project detail

### Commands

- Run: `yarn install` for install all packages
- Run: `yarn dev` for start dev environment
- Run: `yarn build` for build your project
- Run: `yarn start` for start your built project
- Run: `yarn lint` for checking error and fix it

### Project structure

```
├── apis                # All apis come here
├── components          # All components that can share between screen
│   └── _template       # Template for component
├── locales             # I18N Language files
│   ├── en
│   └── vi
├── configs             # All configs and constant goes here
├── hooks               # Custom hooks for project
├── layouts             # Layouts of screen and components
├── pages               # Page file of NextJS (Use as router to screen folder)
├── public              # Public folder, contain static files
├── screens             # Screen component goes here
├── services            # All services goes here
├── states              # State managerment for app
│   └── atoms           # Recoil's atom states (for app state)
├── styles              # App global styles (SCSS)
├── types               # App global type (Typescript)
└── utils               # App's utils
```

### Some code rule

- Component that belong only to screen, should be placed on that screen's folder
- App's state should using Recoil, state relate to API should using Redux
- Function that can be used many time should place in utils or hooks if relate to state
- HOC placed in `layouts` have prefix `*HOC` like `AuthenticationHOC`
- Layout placed in `layouts` and have prefix `*Layout` like `MainLayout` or `DefaultLayout`
- All atom from recoil should have prefix `*Atom` like `DarkmodeAtom`
- All config export from configs folder should have export prefix `*Config` like `BaseConfig` or `ApiConfig`

## Tech included

- `NextJS + Typescript` Base source
- `RecoilJS` State managerment
- `Husky` Git helper
- `SCSS modules` Style system
- `Eslint and Prettier` Rule of code
