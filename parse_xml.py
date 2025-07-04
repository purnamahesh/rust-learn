import xml.etree.ElementTree as ET

tree = ET.parse('/Users/mmv6113/IdeaProjects/data_quality/.run/Main.run.xml')
root = tree.getroot()

for env in root.findall('.//env'):
    name = env.get('name')
    value = env.get('value')
    print(f"{name}={value}")