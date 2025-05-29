# Smoothing: Rust Torch Project

This is a Rust project that uses PyTorch (through `tch-rs`) for AI and machine learning development.

For detailed explanations of smoothing techniques, see:
- [LAPLACE_SMOOTHING.md](readmes/LAPLACE_SMOOTHING.md)
- [LIDSTONE_SMOOTHING.md](readmes/LIDSTONE_SMOOTHING.md)
- [GOOD_TURING_SMOOTHING.md](readmes/GOOD_TURING_SMOOTHING.md)
- [ABSOLUTE_DISCOUNT_SMOOTHING.md](readmes/ABSOLUTE_DISCOUNT_SMOOTHING.md)

## Dataset Setup

To run experiments or train models with this project, you need to download the dataset manually from Kaggle:

- [Email Spam Classification Dataset on Kaggle](https://www.kaggle.com/datasets/purusinghvi/email-spam-classification-dataset)

**Instructions:**
1. Download the dataset from the Kaggle link above (you may need a Kaggle account).
2. Place the downloaded file (e.g., `combined_email_data_sample.csv`) into the `src/assets` directory of this project.

> **Note:** The project expects the dataset to be present in `src/assets` and will not attempt to download it automatically.

## Project Setup

1. Make sure you have Rust installed. If not, install it using:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone this repository:
   ```bash
   git clone <repository-url>
   cd laplace
   ```

3. The project uses specific versions of dependencies, particularly:
   - `tch = "0.19.0"` (PyTorch bindings for Rust)
   Make sure not to change this version as it's compatible with the current libtorch setup.


### Installing libtorch on macOS

The good thing is that there's no need to install libtorch manually and globally. Simply run:
```bash
make setup
```

This will automatically:
1. Download the appropriate version of libtorch for macOS
2. Extract it to the correct location within this project
3. Set up the necessary environment variables in the .cargo/config.toml file

That way this project is self-contained, doesn't require any global installations and you don't have to worry about version conflicts.

## Building the Project

After setting up libtorch and the environment variables, you can build the project:

```bash
cargo build
```

## Project Structure

This project is set up for AI and machine learning development using PyTorch in Rust. It provides a foundation for building:
- Machine Learning models
- Neural Networks
- Deep Learning applications
- AI inference and training pipelines

## Troubleshooting

If you encounter any linking errors open an issue on GitHub and I'll help you out.

## License

MIT License
