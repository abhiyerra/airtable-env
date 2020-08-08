package main

import (
	"fmt"
	"github.com/brianloveswords/airtable"
	flag "github.com/spf13/pflag"
)

var airtableConfig struct {
	ApiKey     string
	BaseKey    string
	Table      string
	View       string
	KeyField   string
	ValueField string
}

type Record struct {
	airtable.Record
	Fields map[string]interface{}
}

func init() {
	flag.StringVar(&airtableConfig.ApiKey, "api-key", "", "Airtable API Key")
	flag.StringVar(&airtableConfig.BaseKey, "base", "", "Airtable Base Key")
	flag.StringVar(&airtableConfig.Table, "table", "", "Airtable Table Name")
	flag.StringVar(&airtableConfig.Table, "key-field", "", "Airtable Key Field Name")
	flag.StringVar(&airtableConfig.Table, "value-field", "", "Airtable Value Field Name")
	flag.StringVar(&airtableConfig.View, "view", "", "Airtable Table View")
}

// tablenv --api --base -- table --key-field --value-field
func main() {
	flag.Parse()

	client := airtable.Client{
		APIKey: airtableConfig.ApiKey,
		BaseID: airtableConfig.BaseKey,
	}

	table := client.Table(airtableConfig.Table)
	records := []Record{}
	table.List(&records, &airtable.Options{
		Fields: []string{airtableConfig.KeyField, airtableConfig.ValueField},
	})

	for _, r := range records {
		fmt.Printf("%s='%s'", r.Fields[airtableConfig.KeyField], r.Fields[airtableConfig.ValueField])
	}
}
