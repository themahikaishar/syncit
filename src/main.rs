use syncit::SyncIt;

fn main() {
    let mut syncit = SyncIt::new(
        "C:/Users/MahiHamim/Desktop/Dev/a",
        "C:/Users/MahiHamim/Desktop/Dev/b",
    );

    syncit.sync();
}
