from bs4 import BeautifulSoup
from requests import get
from csv import writer
import pandas as pd
import re

def get_scores(output_path):
    url = input("Name of Event: ")
    month = str(input("Beginning Month of Search (01, 02...): "))
    year = str(input("Beginning Year to Search (2022, 2023...): "))
    end_year = str(input("End Year "))
    url = url.replace(' ', '%20')
    search = f'https://www.pdga.com/tour/search?OfficialName={url}&date_filter[min][date]={year}-{month}-01&date_filter[max][date]={end_year}-12-31'
    searcher = get(search)
    souper = BeautifulSoup(searcher.text, 'lxml')
    event_nums = set()
    for events in souper.find_all('a', {'href': re.compile('\/t\w*\/e\w*\/\d\d\d\d\d')}):
        event_link = events.get('href')
        event_num = event_link[-5:]
        event_nums.add(event_num)

    for x in event_nums:
        URL = f'https://www.pdga.com/tour/event/{x}'

        r = get(URL)
        soup = BeautifulSoup(r.text, 'lxml')

        # get all tables
        tables = soup.find_all('table')
        titular = str(soup.find('title'))

        title = titular.split('|')[0].replace('<title>','')
        title = title.replace('/', '')
        title = title.replace(' ','_')

        bdf = pd.DataFrame()

        # loop over each table
        for num, table in enumerate(tables, start=1):

            # create filename
            filename = f'{output_path}\\{title}{year}_{num}.csv'

            # open file for writing
            with open(filename, 'w', encoding='utf-8') as f:

                # store rows here
                data = []

                # create csv writer object
                csv_writer = writer(f)

                # go through each row
                rows = table.find_all('tr')
                for row in rows:

                    # write headers if any
                    headers = row.find_all('th')
                    if headers:
                        csv_writer.writerow([header.text.strip() for header in headers])

                    # write column items
                    columns = row.find_all('td')
                    csv_writer.writerow([column.text.strip() for column in columns]) 
            df = pd.read_csv(filename)
            if (list(df.columns.values)[0] == 'Place'):
                bdf = pd.concat([bdf, df])
            else:
                pass

        bdf.to_csv(f'{output_path}\\{title}{year}_merge.csv',index=False)
        path_to_bdf = str(f'{output_path}\\{title}{year}_merge.csv')
        url = url.replace('%20', '_')
        return path_to_bdf, url

def update_tables(path_to_roster, scores_df_path):
    roster = pd.read_csv(path_to_roster, dtype=str)
    scores_df = pd.read_csv(scores_df_path, dtype=str)
    found_players = pd.DataFrame(scores_df[scores_df['PDGA#'].isin(roster['Current PDGA#'])])
    roster = roster.set_index('Current PDGA#')
    found_players = found_players.set_index('PDGA#')
    merge = roster.join(found_players, how='left',lsuffix='_l', rsuffix='_r')
    merge = merge.reset_index()
    merge = merge.sort_values(by=['Team', 'Current Roster #'])
    cols = ['Team', 'PDGA #', 'Drafted', 'Current Roster', 'Current PDGA#', 'Current Roster #', 'Place']
    listy = merge.columns.to_list()
    listy2 = list(set(listy).difference(cols))
    for x in listy2:
        del merge[x]
    merge['Place'].fillna('DNC/DNF',inplace=True)
    print(merge)
    merge = merge[['Team', 'PDGA #', 'Drafted', 'Current Roster', 'Current Roster #', 'Place']]
    return merge

def main():
    path = input("Output Path? > ")
    table_df, url = get_scores(path)
    roster_path = input("Path to Roster? > ")
    new_table = update_tables(roster_path, table_df)
    out = path + f'\\{url}_roster_updated'
    new_table.to_csv(out+'.csv')

if __name__ == '__main__':
    main()