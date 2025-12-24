This script is designed to be the "Business Intelligence" engine for your Ejected Media universe. By feeding your Launch Cart sales data into this, you can visually determine where to focus your development for localSq, Shop Oahu, and Phoenix Valley.
The "Space Center" Sales Mapping Script
This script does three things:
 * Reads your exported CSV from Launch Cart.
 * Geocodes zip codes into Latitude/Longitude (using the fast, offline pgeocode library).
 * Generates an interactive HTML heatmap you can open in any browser.
<!-- end list -->

```
import pandas as pd
import pgeocode
import folium
from folium.plugins import HeatMap

# 1. Load your Launch Cart export
# Replace 'sales_export.csv' with your actual filename
def generate_sales_map(file_path, output_name="space_center_market_map.html"):
    try:
        df = pd.read_csv(file_path)
        
        # Ensure the zip code column is treated as a string to preserve leading zeros
        # Adjust 'Zip Code' to match your CSV's column name
        zip_col = 'Zip Code' 
        df[zip_col] = df[zip_col].astype(str).str.zfill(5)
        
        print(f"Processing {len(df)} sales records...")

        # 2. Initialize Geocoder (US focus)
        nomi = pgeocode.Nominatim('us')
        
        # Batch geocode the unique zip codes to save time
        unique_zips = df[zip_col].unique()
        geo_data = nomi.query_postal_code(unique_zips)
        
        # Merge coordinates back into the main sales dataframe
        df = df.merge(geo_data[['postal_code', 'latitude', 'longitude']], 
                     left_on=zip_col, right_on='postal_code', how='left')

        # Drop rows that couldn't be geocoded
        df = df.dropna(subset=['latitude', 'longitude'])

        # 3. Create the Interactive Map
        # Start the map center at a neutral US point or specific HQ
        m = folium.Map(location=[37.0902, -95.7129], zoom_start=4, tiles="CartoDB positron")

        # Prepare data for HeatMap: [[lat, lon, weight], ...]
        # Here we use the frequency of the zip code as the 'weight'
        heat_data = df[['latitude', 'longitude']].values.tolist()

        HeatMap(heat_data, radius=15, blur=10).add_to(m)

        # 4. Save and Finish
        m.save(output_name)
        print(f"Success! Map saved to {output_name}")
        
    except Exception as e:
        print(f"Error: {e}")
```

# Run the function
# generate_sales_map('your_launch_cart_file.csv')

How this fuels your projects:
 * For localSq (Phoenix Focus): When you run this, zoom into the Phoenix area. If you see "hot spots" in Tempe vs. Scottsdale, that tells you which local sub-communities in localSq should receive the first beta invites or localized "Space Center" ad placements.
 * For Shop Oahu: Since you have a specific focus on Hawaii, this map will immediately show you the density of your "Littles" community across the islands. You can use these coordinates to decide where to place "Virtual Pop-ups" or real-world QR code stickers for Shop Oahu.
 * For Ejected Media Growth: This script turns "Customer 123 in Honolulu" into a strategic data point. It allows you to prove market demand when you're looking for partnerships or scaling the "universe" further.
Quick Tip: Before running this locally, you’ll need to install the libraries via terminal:
pip install pandas pgeocode folium
Would you like me to add a feature to this script that filters the sales specifically for Oahu or Phoenix so you can generate separate reports for those specific projects?
