// Copyright 2018 foundationdb-rs developers, https://github.com/bluejekyll/foundationdb-rs/graphs/contributors
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use foundationdb::*;

mod common;

async fn test_watch_async() -> error::Result<()> {
    const KEY: &'static [u8] = b"test-watch";

    let db = Database::default()?;

    eprintln!("setting watch");
    let trx = db.create_trx()?;
    let watch = trx.watch(KEY);
    trx.commit().await?;
    eprintln!("watch committed");

    eprintln!("writing value");
    let trx = db.create_trx()?;
    let value = common::random_str(10);
    trx.set(KEY, value.as_bytes());
    trx.commit().await?;
    eprintln!("write committed");

    watch.await?;

    Ok(())
}

#[test]
fn test_watch() {
    common::boot();
    futures::executor::block_on(test_watch_async()).expect("failed to run");
}

async fn test_watch_without_commit_async() -> error::Result<()> {
    const KEY: &'static [u8] = b"test-watch-2";

    let db = Database::default()?;

    eprintln!("setting watch");
    let trx = db.create_trx()?;
    let watch = trx.watch(KEY);

    drop(trx);
    assert!(watch.await.is_err());

    Ok(())
}

#[test]
fn test_watch_without_commit() {
    common::boot();
    futures::executor::block_on(test_watch_without_commit_async()).expect("failed to run");
}
