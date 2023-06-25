#include <iostream> 
#include <string> 
#include <map> 

using namespace std; 

class AddictionRecoverySupportGroup 
{ 
    private: 
        map<string, string> people; 
  
    public: 
        AddictionRecoverySupportGroup() 
        { 
            people = {}; 
        } 
  
        void addPerson(string name, string address) 
        { 
            people[name] = address; 
        } 
  
        void printPerson(string name) 
        { 
            cout << name << " lives at " << people[name] << endl; 
        } 
  
        void printAll() 
        { 
            for (auto it = people.begin(); it != people.end(); it++) 
            { 
                cout << it->first << " lives at " << it->second << endl; 
            } 
        } 
}; 

int main() 
{
    AddictionRecoverySupportGroup arsg; 
  
    arsg.addPerson("John", "56th Sreet"); 
    arsg.addPerson("Kelly", "2nd Avenue"); 
    arsg.addPerson("Donald", "3rd Avenue"); 
    arsg.addPerson("Mary", "7th Avenue"); 
  
    arsg.printPerson("John"); 
    arsg.printPerson("Kelly"); 
  
    arsg.printAll(); 
}