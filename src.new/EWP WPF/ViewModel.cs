using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Globalization;
using System.Windows;
using System.Windows.Controls;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using DocumentFormat.OpenXml.Bibliography;
using Windows.System;

namespace EWP_WPF
{
    public partial class ViewModel: ObservableRecipient
    {
        [ObservableProperty]
        private string? tbname;

        [ObservableProperty]
        private string? homeTab = "EWP Office";

        [ObservableProperty]
        private string? newbtn = "New";

        [ObservableProperty]
        private ObservableCollection<TabItem> tabItems;

        [RelayCommand]
        public void NewTab()
        {
            var StyleTabItem = new ResourceDictionary
            {
                Source = new Uri("Styles.xaml", UriKind.RelativeOrAbsolute)
            }["TabItemStyle"] as Style;
            var tabitem = new TabItem();
            tabitem.Content = new UserControl2();
            tabItems.Add(tabitem);
            tabitem.Header = tbname;
            tabitem.Style = StyleTabItem;
        }

        public ViewModel() {
            Init();
            DecideLang();
        }

        void DecideLang()
        {
            if (CultureInfo.CurrentCulture.Name == "zh-CN")
            {
                newbtn = zhCNLang["New"];
            }
            else
            {
                newbtn = enUKLang["New"];
            }
        }
    }
    partial class ViewModel
    {
        private Dictionary<string, string>? enUKLang = [];
        private Dictionary<string, string>? zhCNLang = [];

        void Init() 
        {
            enUKLang.Add("New", "New");
            zhCNLang.Add("New", "新建");
        }
    }
}