import os
import subprocess


TEMPLATE: str = '            <a href="wheels/{0}" class="link" onmouseover="style=\'text-decoration:underline\'" onmouseout="style=\'text-decoration:none\'"><span style="font-size: x-large;">{0}</span></a>'

def main() -> None:
    with open('site/index.html', 'r') as f:
        current = f.read()
    
    with open('site/index.html', 'w') as f:
        wheels = [TEMPLATE.format(wheel) for wheel in os.listdir('site/wheels')]
        long_hash = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True).stdout.decode('utf-8')
        short_hash = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True).stdout.decode('utf-8')

        f.write(current.replace('<!-- TO REPLACE -->', '\n'.join(wheels)).replace('short_hash', short_hash).replace('long_hash', long_hash))

if __name__ == '__main__':
    main()
