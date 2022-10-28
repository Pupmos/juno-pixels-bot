# NoPixel Drawing Bøt 4 nopixels.io ❤️
> Created wif lovb by Pupmøs 

## Requirements 
<!-- Rust requirements -->
- [Rust](https://www.rust-lang.org/tools/install)
- Throwaway 24 word mnemonic for a Junø Wallet
  - Do not reuse an existing wallet.


## Instructions 
<!-- clone repo -->
- Clone this repo
```bash
git clone https://github.com/Pupmos/juno-pixels-bot.git
```
<!-- insert image into images folder -->
- Insert your image into the `images` folder

<!-- create a .env file -->
- Create a `.env` file in the root directory
```bash
touch .env
```

<!-- add your throwaway mnemonic to the .env file -->
- Add your throwaway mnemonic to the `.env` file
```bash
MNEMONIC="your throwaway mnemonic"
```

<!-- add your image to the env file -->
- Add your image to the `.env` file
```bash
IMAGE="./images/your-image.jpg"
```

<!-- add square_x and square_y to env file -->
- Add `SQUARE_X` and `SQUARE_Y` to the `.env` file
  - the pixel grid is broken into 35x35 chunks, numbered top to bottom and left to right (see .env.example for all corners)
```bash
# top left square
SQUARE_X=0
SQUARE_Y=1
```
