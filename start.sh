cd ~/Documents/programming/KTUG-ChessBot/
git fetch
git pull
source .venv/bin/activate
python run.py &

sleep 1

cd ~/Documents/programming/rusty_chess/
cargo run --release
