# Img File Format - README

## Overview

Welcome to the **Img File Format** project! This project provides a custom image format called `.silly` along with a set of tools to convert between `.silly` and `.jpg/.jpeg` formats. It also includes a GUI application built with `egui` that allows users to view images in the `.silly` format.

## Features

- **Custom Image Format (`.silly`)**: A unique image format that stores pixel data as a hexadecimal string.
- **Conversion Utilities**: Convert images between `.silly` and `.jpg/.jpeg` formats.
- **Image Viewer**: A simple GUI application to view images in the `.silly` format.

## Prerequisites

Before setting up the project, ensure that you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install): The Rust programming language and its package manager, Cargo.
- A compatible C++ compiler (required by Skia and other native dependencies).

## Installation

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/Braeulias/Silly_ImageFile_Forma.git
   cd img-file-format
   ```

2. **Build the Project:**

   Run the following command to build the project:

   ```sh
   cargo build --release
   ```

## Usage

### 1. Converting Images

The project allows you to convert images between `.silly` and `.jpg/.jpeg` formats.

- **Convert `.jpg/.jpeg` to `.silly`:**

  ```sh
  cargo run conv path/to/your-image.jpg
  ```

  This will create a new file with the `.silly` extension in the same directory.

- **Convert `.silly` to `.jpg/.jpeg`:**

  ```sh
  cargo run conv path/to/your-image.silly
  ```

  This will create a new file with the `.jpg` extension in the same directory.

### 2. Viewing `.silly` Images

You can view images in the `.silly` format using the included GUI application.

- **View a `.silly` Image:**

  ```sh
  cargo run path/to/your-image.silly
  ```

  The application will open a window displaying the image.

## Project Structure

- **`src/`**: Contains the Rust source code for the project.
  - **`main.rs`**: The main entry point for the application and utility functions.
  - **`img_format.rs`**: Utility functions for converting between image formats.
  - **`viewer.rs`**: The GUI application for viewing `.silly` images.

## Known Issues

- The current implementation expects images with a correct `.silly` format. Invalid or corrupted files may cause unexpected behavior.
- Conversion between formats might fail if the input image size exceeds certain limits.

## Contributions

Contributions are welcome! If you have any ideas for improving the project or find a bug, feel free to open an issue or submit a pull request.

## License



---

Thank you for using the **Img File Format** project! If you have any questions or need further assistance, please don't hesitate to reach out. Happy coding!
