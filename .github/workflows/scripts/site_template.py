import os


TEMPLATE: str = '            <a href="wheels/{0}" class="link"><span style="font-size: x-large;">{0}</span></a>'

def main() -> None:
    with open('site/index.html', 'r') as f:
        current = f.read()
    
    with open('site/index.html', 'w') as f:
        wheels = [TEMPLATE.format(wheel) for wheel in os.listdir('site/wheels')]

        f.write(current.replace('<!-- TO REPLACE -->', '\n'.join(wheels)))

if __name__ == '__main__':
    main()
