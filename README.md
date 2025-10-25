# SOEN342_Project 
## Team Members  
| Name                  | Student ID |
|------------------------|------------|
| **Michael Pouget**     | 40246798   |
| **Andrea Torres**       | 40289711   |
| **Thi Hong Mai Nguyen** | 40248343   |

Installation:

Backend (Ubuntu):
sudo apt update
sudo apt install build-essential pkg-config libssl-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
cargo fetch

Frontend:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
source ~/.bashrc
nvm install --lts
nvm use --lts
npm install

Compile:

Backend:
cargo fetch

Frontend:
npm run build

Run:

Backend:
cargo run

Frontend:
npm start


Dependencies needed to install:

Backend:
http = "1"
uuid = {version = "1.8", features = ["v4", "serde"]}
chrono = {version = "0.4.42", features = ["serde"]}
csv = "1.3.1"
serde = {version = "1.0.226", features = ["derive"]}
axum = {version = "0.8.6", features = ["multipart"]}
tokio = {version = "1", features = ["full"]}
serde_json = "1.0.145"
strum = "0.27"
strum_macros = "0.27"
tower = "0.5.2"
tower-http = {version = "0.6.6", features = ["cors", "trace"]}
tracing = "0.1"
tracing-subscriber = {version = "0.3", features = ["env-filter"]}
once_cell = "1.21.3"
sqlx = {version = "0.8.6", features = ["runtime-tokio", "chrono", "postgres", "runtime-tokio-native-tls", "uuid", "macros"]}
dotenvy = "0.15"

Frontend:
{
  "name": "SOEN342_project",
  "lockfileVersion": 3,
  "requires": true,
  "packages": {
    "": {
      "dependencies": {
        "@types/react-datepicker": "^7.0.0",
        "react-datepicker": "^8.8.0"
      }
    },
    "node_modules/@floating-ui/core": {
      "version": "1.7.3",
      "resolved": "https://registry.npmjs.org/@floating-ui/core/-/core-1.7.3.tgz",
      "integrity": "sha512-sGnvb5dmrJaKEZ+LDIpguvdX3bDlEllmv4/ClQ9awcmCZrlx5jQyyMWFM5kBI+EyNOCDDiKk8il0zeuX3Zlg/w==",
      "dependencies": {
        "@floating-ui/utils": "^0.2.10"
      }
    },
    "node_modules/@floating-ui/dom": {
      "version": "1.7.4",
      "resolved": "https://registry.npmjs.org/@floating-ui/dom/-/dom-1.7.4.tgz",
      "integrity": "sha512-OOchDgh4F2CchOX94cRVqhvy7b3AFb+/rQXyswmzmGakRfkMgoWVjfnLWkRirfLEfuD4ysVW16eXzwt3jHIzKA==",
      "dependencies": {
        "@floating-ui/core": "^1.7.3",
        "@floating-ui/utils": "^0.2.10"
      }
    },
    "node_modules/@floating-ui/react": {
      "version": "0.27.16",
      "resolved": "https://registry.npmjs.org/@floating-ui/react/-/react-0.27.16.tgz",
      "integrity": "sha512-9O8N4SeG2z++TSM8QA/KTeKFBVCNEz/AGS7gWPJf6KFRzmRWixFRnCnkPHRDwSVZW6QPDO6uT0P2SpWNKCc9/g==",
      "dependencies": {
        "@floating-ui/react-dom": "^2.1.6",
        "@floating-ui/utils": "^0.2.10",
        "tabbable": "^6.0.0"
      },
      "peerDependencies": {
        "react": ">=17.0.0",
        "react-dom": ">=17.0.0"
      }
    },
    "node_modules/@floating-ui/react-dom": {
      "version": "2.1.6",
      "resolved": "https://registry.npmjs.org/@floating-ui/react-dom/-/react-dom-2.1.6.tgz",
      "integrity": "sha512-4JX6rEatQEvlmgU80wZyq9RT96HZJa88q8hp0pBd+LrczeDI4o6uA2M+uvxngVHo4Ihr8uibXxH6+70zhAFrVw==",
      "dependencies": {
        "@floating-ui/dom": "^1.7.4"
      },
      "peerDependencies": {
        "react": ">=16.8.0",
        "react-dom": ">=16.8.0"
      }
    },
    "node_modules/@floating-ui/utils": {
      "version": "0.2.10",
      "resolved": "https://registry.npmjs.org/@floating-ui/utils/-/utils-0.2.10.tgz",
      "integrity": "sha512-aGTxbpbg8/b5JfU1HXSrbH3wXZuLPJcNEcZQFMxLs3oSzgtVu6nFPkbbGGUvBcUjKV2YyB9Wxxabo+HEH9tcRQ=="
    },
    "node_modules/@types/react-datepicker": {
      "version": "7.0.0",
      "resolved": "https://registry.npmjs.org/@types/react-datepicker/-/react-datepicker-7.0.0.tgz",
      "integrity": "sha512-4tWwOUq589tozyQPBVEqGNng5DaZkomx5IVNuur868yYdgjH6RaL373/HKiVt1IDoNNXYiTGspm1F7kjrarM8Q==",
      "deprecated": "This is a stub types definition. react-datepicker provides its own type definitions, so you do not need this installed.",
      "dependencies": {
        "react-datepicker": "*"
      }
    },
    "node_modules/clsx": {
      "version": "2.1.1",
      "resolved": "https://registry.npmjs.org/clsx/-/clsx-2.1.1.tgz",
      "integrity": "sha512-eYm0QWBtUrBWZWG0d386OGAw16Z995PiOVo2B7bjWSbHedGl5e0ZWaq65kOGgUSNesEIDkB9ISbTg/JK9dhCZA==",
      "engines": {
        "node": ">=6"
      }
    },
    "node_modules/date-fns": {
      "version": "4.1.0",
      "resolved": "https://registry.npmjs.org/date-fns/-/date-fns-4.1.0.tgz",
      "integrity": "sha512-Ukq0owbQXxa/U3EGtsdVBkR1w7KOQ5gIBqdH2hkvknzZPYvBxb/aa6E8L7tmjFtkwZBu3UXBbjIgPo/Ez4xaNg==",
      "funding": {
        "type": "github",
        "url": "https://github.com/sponsors/kossnocorp"
      }
    },
    "node_modules/react": {
      "version": "19.2.0",
      "resolved": "https://registry.npmjs.org/react/-/react-19.2.0.tgz",
      "integrity": "sha512-tmbWg6W31tQLeB5cdIBOicJDJRR2KzXsV7uSK9iNfLWQ5bIZfxuPEHp7M8wiHyHnn0DD1i7w3Zmin0FtkrwoCQ==",
      "peer": true,
      "engines": {
        "node": ">=0.10.0"
      }
    },
    "node_modules/react-datepicker": {
      "version": "8.8.0",
      "resolved": "https://registry.npmjs.org/react-datepicker/-/react-datepicker-8.8.0.tgz",
      "integrity": "sha512-rIJLhww1B5cQY7GYEfSEXvldlGp+GIVU5oE7lxqeK4fmdv5F9bVndplDmblMCvfSMazXmeJ6OHBvRs/PkEhwUQ==",
      "dependencies": {
        "@floating-ui/react": "^0.27.15",
        "clsx": "^2.1.1",
        "date-fns": "^4.1.0"
      },
      "peerDependencies": {
        "react": "^16.9.0 || ^17 || ^18 || ^19 || ^19.0.0-rc",
        "react-dom": "^16.9.0 || ^17 || ^18 || ^19 || ^19.0.0-rc"
      }
    },
    "node_modules/react-dom": {
      "version": "19.2.0",
      "resolved": "https://registry.npmjs.org/react-dom/-/react-dom-19.2.0.tgz",
      "integrity": "sha512-UlbRu4cAiGaIewkPyiRGJk0imDN2T3JjieT6spoL2UeSf5od4n5LB/mQ4ejmxhCFT1tYe8IvaFulzynWovsEFQ==",
      "peer": true,
      "dependencies": {
        "scheduler": "^0.27.0"
      },
      "peerDependencies": {
        "react": "^19.2.0"
      }
    },
    "node_modules/scheduler": {
      "version": "0.27.0",
      "resolved": "https://registry.npmjs.org/scheduler/-/scheduler-0.27.0.tgz",
      "integrity": "sha512-eNv+WrVbKu1f3vbYJT/xtiF5syA5HPIMtf9IgY/nKg0sWqzAUEvqY/xm7OcZc/qafLx/iO9FgOmeSAp4v5ti/Q==",
      "peer": true
    },
    "node_modules/tabbable": {
      "version": "6.3.0",
      "resolved": "https://registry.npmjs.org/tabbable/-/tabbable-6.3.0.tgz",
      "integrity": "sha512-EIHvdY5bPLuWForiR/AN2Bxngzpuwn1is4asboytXtpTgsArc+WmSJKVLlhdh71u7jFcryDqB2A8lQvj78MkyQ=="
    }
  }
}
