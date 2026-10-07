#[doc = "Register `POTRA` reader"]
pub type R = crate::R<PotraSpec>;
#[doc = "Register `POTRA` writer"]
pub type W = crate::W<PotraSpec>;
#[doc = "Field `POT00` reader - desc POT00"]
pub type Pot00R = crate::BitReader;
#[doc = "Field `POT00` writer - desc POT00"]
pub type Pot00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT01` reader - desc POT01"]
pub type Pot01R = crate::BitReader;
#[doc = "Field `POT01` writer - desc POT01"]
pub type Pot01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT02` reader - desc POT02"]
pub type Pot02R = crate::BitReader;
#[doc = "Field `POT02` writer - desc POT02"]
pub type Pot02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT03` reader - desc POT03"]
pub type Pot03R = crate::BitReader;
#[doc = "Field `POT03` writer - desc POT03"]
pub type Pot03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT04` reader - desc POT04"]
pub type Pot04R = crate::BitReader;
#[doc = "Field `POT04` writer - desc POT04"]
pub type Pot04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT05` reader - desc POT05"]
pub type Pot05R = crate::BitReader;
#[doc = "Field `POT05` writer - desc POT05"]
pub type Pot05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT06` reader - desc POT06"]
pub type Pot06R = crate::BitReader;
#[doc = "Field `POT06` writer - desc POT06"]
pub type Pot06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT07` reader - desc POT07"]
pub type Pot07R = crate::BitReader;
#[doc = "Field `POT07` writer - desc POT07"]
pub type Pot07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT08` reader - desc POT08"]
pub type Pot08R = crate::BitReader;
#[doc = "Field `POT08` writer - desc POT08"]
pub type Pot08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT09` reader - desc POT09"]
pub type Pot09R = crate::BitReader;
#[doc = "Field `POT09` writer - desc POT09"]
pub type Pot09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT10` reader - desc POT10"]
pub type Pot10R = crate::BitReader;
#[doc = "Field `POT10` writer - desc POT10"]
pub type Pot10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT11` reader - desc POT11"]
pub type Pot11R = crate::BitReader;
#[doc = "Field `POT11` writer - desc POT11"]
pub type Pot11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT12` reader - desc POT12"]
pub type Pot12R = crate::BitReader;
#[doc = "Field `POT12` writer - desc POT12"]
pub type Pot12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT13` reader - desc POT13"]
pub type Pot13R = crate::BitReader;
#[doc = "Field `POT13` writer - desc POT13"]
pub type Pot13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT14` reader - desc POT14"]
pub type Pot14R = crate::BitReader;
#[doc = "Field `POT14` writer - desc POT14"]
pub type Pot14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POT15` reader - desc POT15"]
pub type Pot15R = crate::BitReader;
#[doc = "Field `POT15` writer - desc POT15"]
pub type Pot15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POT00"]
    #[inline(always)]
    pub fn pot00(&self) -> Pot00R {
        Pot00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POT01"]
    #[inline(always)]
    pub fn pot01(&self) -> Pot01R {
        Pot01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POT02"]
    #[inline(always)]
    pub fn pot02(&self) -> Pot02R {
        Pot02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POT03"]
    #[inline(always)]
    pub fn pot03(&self) -> Pot03R {
        Pot03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POT04"]
    #[inline(always)]
    pub fn pot04(&self) -> Pot04R {
        Pot04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POT05"]
    #[inline(always)]
    pub fn pot05(&self) -> Pot05R {
        Pot05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POT06"]
    #[inline(always)]
    pub fn pot06(&self) -> Pot06R {
        Pot06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POT07"]
    #[inline(always)]
    pub fn pot07(&self) -> Pot07R {
        Pot07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POT08"]
    #[inline(always)]
    pub fn pot08(&self) -> Pot08R {
        Pot08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POT09"]
    #[inline(always)]
    pub fn pot09(&self) -> Pot09R {
        Pot09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POT10"]
    #[inline(always)]
    pub fn pot10(&self) -> Pot10R {
        Pot10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POT11"]
    #[inline(always)]
    pub fn pot11(&self) -> Pot11R {
        Pot11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POT12"]
    #[inline(always)]
    pub fn pot12(&self) -> Pot12R {
        Pot12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POT13"]
    #[inline(always)]
    pub fn pot13(&self) -> Pot13R {
        Pot13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POT14"]
    #[inline(always)]
    pub fn pot14(&self) -> Pot14R {
        Pot14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POT15"]
    #[inline(always)]
    pub fn pot15(&self) -> Pot15R {
        Pot15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POT00"]
    #[inline(always)]
    pub fn pot00(&mut self) -> Pot00W<'_, PotraSpec> {
        Pot00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POT01"]
    #[inline(always)]
    pub fn pot01(&mut self) -> Pot01W<'_, PotraSpec> {
        Pot01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POT02"]
    #[inline(always)]
    pub fn pot02(&mut self) -> Pot02W<'_, PotraSpec> {
        Pot02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc POT03"]
    #[inline(always)]
    pub fn pot03(&mut self) -> Pot03W<'_, PotraSpec> {
        Pot03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc POT04"]
    #[inline(always)]
    pub fn pot04(&mut self) -> Pot04W<'_, PotraSpec> {
        Pot04W::new(self, 4)
    }
    #[doc = "Bit 5 - desc POT05"]
    #[inline(always)]
    pub fn pot05(&mut self) -> Pot05W<'_, PotraSpec> {
        Pot05W::new(self, 5)
    }
    #[doc = "Bit 6 - desc POT06"]
    #[inline(always)]
    pub fn pot06(&mut self) -> Pot06W<'_, PotraSpec> {
        Pot06W::new(self, 6)
    }
    #[doc = "Bit 7 - desc POT07"]
    #[inline(always)]
    pub fn pot07(&mut self) -> Pot07W<'_, PotraSpec> {
        Pot07W::new(self, 7)
    }
    #[doc = "Bit 8 - desc POT08"]
    #[inline(always)]
    pub fn pot08(&mut self) -> Pot08W<'_, PotraSpec> {
        Pot08W::new(self, 8)
    }
    #[doc = "Bit 9 - desc POT09"]
    #[inline(always)]
    pub fn pot09(&mut self) -> Pot09W<'_, PotraSpec> {
        Pot09W::new(self, 9)
    }
    #[doc = "Bit 10 - desc POT10"]
    #[inline(always)]
    pub fn pot10(&mut self) -> Pot10W<'_, PotraSpec> {
        Pot10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc POT11"]
    #[inline(always)]
    pub fn pot11(&mut self) -> Pot11W<'_, PotraSpec> {
        Pot11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc POT12"]
    #[inline(always)]
    pub fn pot12(&mut self) -> Pot12W<'_, PotraSpec> {
        Pot12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc POT13"]
    #[inline(always)]
    pub fn pot13(&mut self) -> Pot13W<'_, PotraSpec> {
        Pot13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POT14"]
    #[inline(always)]
    pub fn pot14(&mut self) -> Pot14W<'_, PotraSpec> {
        Pot14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POT15"]
    #[inline(always)]
    pub fn pot15(&mut self) -> Pot15W<'_, PotraSpec> {
        Pot15W::new(self, 15)
    }
}
#[doc = "desc POTRA\n\nYou can [`read`](crate::Reg::read) this register and get [`potra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`potra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PotraSpec;
impl crate::RegisterSpec for PotraSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`potra::R`](R) reader structure"]
impl crate::Readable for PotraSpec {}
#[doc = "`write(|w| ..)` method takes [`potra::W`](W) writer structure"]
impl crate::Writable for PotraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POTRA to value 0"]
impl crate::Resettable for PotraSpec {}
