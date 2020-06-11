import yfinance as yf

msft = yf.Ticker("MSFT")
stock = yf.Ticker('AMD')

# get stock info
msft.info

# get historical market data
hist = msft.history(period="max")

print(hist)

# show actions (dividends, splits)
print(msft.actions)

# show dividends
print(msft.dividends)

# show splits
print(msft.splits)

# show financials
print(msft.financials)
print(msft.quarterly_financials)

# show major holders
stock.major_holders

# show institutional holders
stock.institutional_holders

# show balance heet
msft.balance_sheet
msft.quarterly_balance_sheet

# show cashflow
msft.cashflow
msft.quarterly_cashflow

# show earnings
msft.earnings
msft.quarterly_earnings

# show sustainability
print(msft.sustainability)

# show analysts recommendations
msft.recommendations

# show next event (earnings, etc)
msft.calendar

# show ISIN code - *experimental*
# ISIN = International Securities Identification Number
msft.isin

# show options expirations
msft.options

# get option chain for specific expiration
opt = msft.option_chain('2021-01-01')
# data available via: opt.calls, opt.puts