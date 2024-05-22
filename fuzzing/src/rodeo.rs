use honggfuzz::fuzz;
use lasso2::Rodeo;

fn main() {
    let mut rodeo = Rodeo::default();

    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(string) = std::str::from_utf8(data) {
                if let Some(key) = rodeo.try_get_or_intern(string) {
                    assert_eq!(string, rodeo.resolve(&key));
                    assert_eq!(Some(key), rodeo.get(string));
                }

                for (key, string) in rodeo.iter() {
                    assert_eq!(string, rodeo.resolve(&key));
                    assert_eq!(Some(string), rodeo.try_resolve(&key));
                    unsafe { assert_eq!(string, rodeo.resolve_unchecked(&key)) };
                    assert_eq!(Some(key), rodeo.get(string));
                }
            } else {
                rodeo = Rodeo::default();
            }
        });
    }
}
