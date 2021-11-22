import os, json

os.chdir("dev/db-seeder/data")

with open("first_names.json") as f:
    fnames = json.load(f)

with open("last_names.json") as f: 
    lnames = json.load(f)

with open("names.json", "w") as f:
    names = []

    fcount = 0
    lcount = 0
    while True:

        

        fname = fnames["data"][fcount]
        lname = lnames["data"][lcount]

        names.append({
            "first_name": fname,
            "last_name": lname
        })

        fcount += 1
        lcount += 1

        if fcount >= len(fnames["data"]):
            break

        if lcount >= len(lnames["data"]):
            lcount = 0        
    
    json.dump(names, f, indent=4)