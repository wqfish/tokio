(function() {var implementors = {};
implementors["tokio"] = [{text:"impl&lt;'_, S:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/io/struct.Seek.html\" title=\"struct tokio::io::Seek\">Seek</a>&lt;'_, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"tokio/io/trait.AsyncSeek.html\" title=\"trait tokio::io::AsyncSeek\">AsyncSeek</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tokio::io::seek::Seek"]},{text:"impl&lt;'_, R:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>, W:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/io/struct.Copy.html\" title=\"struct tokio::io::Copy\">Copy</a>&lt;'_, R, W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"tokio/io/trait.AsyncRead.html\" title=\"trait tokio::io::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"tokio/io/trait.AsyncWrite.html\" title=\"trait tokio::io::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>",synthetic:false,types:["tokio::io::util::copy::Copy"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/process/struct.Child.html\" title=\"struct tokio::process::Child\">Child</a>",synthetic:false,types:["tokio::process::Child"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/sync/oneshot/struct.Receiver.html\" title=\"struct tokio::sync::oneshot::Receiver\">Receiver</a>&lt;T&gt;",synthetic:false,types:["tokio::sync::oneshot::Receiver"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/task/struct.JoinHandle.html\" title=\"struct tokio::task::JoinHandle\">JoinHandle</a>&lt;T&gt;",synthetic:false,types:["tokio::task::join::JoinHandle"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/task/struct.LocalSet.html\" title=\"struct tokio::task::LocalSet\">LocalSet</a>",synthetic:false,types:["tokio::task::local::LocalSet"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/time/struct.Delay.html\" title=\"struct tokio::time::Delay\">Delay</a>",synthetic:false,types:["tokio::time::delay::Delay"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio/time/struct.Timeout.html\" title=\"struct tokio::time::Timeout\">Timeout</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,&nbsp;</span>",synthetic:false,types:["tokio::time::timeout::Timeout"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()