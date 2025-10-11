# SOEN342_Project 
## Team Members  
| Name                  | Student ID |
|------------------------|------------|
| **Michael Pouget**     | 40246798   |
| **Andrea Torres**       | 40289711   |
| **Thi Hong Mai Nguyen** | 40248343   |

Dependencies needed:


backend:

http = "1"
chrono = { version = "0.4.42", features = ["serde"] }
csv = "1.3.1"
serde = { version = "1.0.226", features = ["derive"] }
axum = { version = "0.8.6", features = ["multipart"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0.145"
strum = "0.27"
strum_macros = "0.27"
tower = "0.5.2"
tower-http = { version = "0.6.6", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
once_cell = "1.21.3"

frontend:

{
  "name": "frontend",
  "version": "0.1.0",
  "private": true,
  "proxy": "http://localhost:3000",
  "dependencies": {
    "@reduxjs/toolkit": "^2.9.0",
    "@testing-library/dom": "^10.4.1",
    "@testing-library/jest-dom": "^6.8.0",
    "@testing-library/react": "^16.3.0",
    "@testing-library/user-event": "^13.5.0",
    "@types/jest": "^27.5.2",
    "@types/node": "^16.18.126",
    "@types/react": "^19.1.13",
    "@types/react-dom": "^19.1.9",
    "axios": "^1.12.2",
    "classnames": "^2.5.1",
    "react": "^19.1.1",
    "react-dom": "^19.1.1",
    "react-router-dom": "^7.9.2",
    "react-scripts": "^5.0.1",
    "typescript": "^4.9.5",
    "web-vitals": "^2.1.4"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app",
      "react-app/jest"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}
