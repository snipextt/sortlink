# sortlink

A shortlink utility

Create a new shortlink
```bash
curl -X POST https://shortner.shuttle.rs
   -H "Content-Type: application/json"
   -d '{"shortlink": "your shortlink", "link": "the long link"}'  
```

Visit your shortlink - ``https://shortner.shuttle.rs/<your shortlink>``
