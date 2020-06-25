macro_rules! escape_impl {
    (@inner [$dollar:tt] $name:ident; $($l:expr => $r:expr),+ $(,)*) => {
        macro_rules! $name {
            ($dollar e:expr) => {
                match $dollar e {
                    $($l => break $r,)+
                    _ => (),
                }
            };
            ($dollar e:expr, $dollar r:ident) => {
                match $dollar e {
                    $($l => {
                        $dollar r = $r;
                        break;
                    })+
                    _ => (),
                }
            };
            (vec $dollar e:expr, $dollar v:ident) => {
                match $dollar e {
                    $($l => $dollar v.extend_from_slice($r),)+
                    _ => $dollar v.push($dollar e),
                }
            };
            (vec $dollar e:expr, $dollar v:ident, $dollar r:block) => {
                match $dollar e {
                    $($l => $dollar v.extend_from_slice($r),)+
                    _ => $dollar r,
                }
            };
            (writer $dollar e:expr, $dollar w:ident) => {
                match $dollar e {
                    $($l => $dollar w.write_all($r)?,)+
                    _ => $dollar w.write_all(&[$dollar e])?,
                }
            };
            (writer $dollar e:expr, $dollar w:ident, $dollar r:block) => {
                match $dollar e {
                    $($l => $dollar w.write_all($r)?,)+
                    _ => $dollar r,
                }
            };
        }
    };
    ($name:ident; $($l:expr => $r:expr),+ $(,)*) => {
        escape_impl! {
            @inner [$]
            $name;
            $($l => $r.as_ref(),)*
        }
    };
}
