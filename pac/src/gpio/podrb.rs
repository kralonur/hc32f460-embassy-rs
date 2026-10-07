#[doc = "Register `PODRB` reader"]
pub type R = crate::R<PodrbSpec>;
#[doc = "Register `PODRB` writer"]
pub type W = crate::W<PodrbSpec>;
#[doc = "Field `POUT00` reader - desc POUT00"]
pub type Pout00R = crate::BitReader;
#[doc = "Field `POUT00` writer - desc POUT00"]
pub type Pout00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT01` reader - desc POUT01"]
pub type Pout01R = crate::BitReader;
#[doc = "Field `POUT01` writer - desc POUT01"]
pub type Pout01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT02` reader - desc POUT02"]
pub type Pout02R = crate::BitReader;
#[doc = "Field `POUT02` writer - desc POUT02"]
pub type Pout02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT03` reader - desc POUT03"]
pub type Pout03R = crate::BitReader;
#[doc = "Field `POUT03` writer - desc POUT03"]
pub type Pout03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT04` reader - desc POUT04"]
pub type Pout04R = crate::BitReader;
#[doc = "Field `POUT04` writer - desc POUT04"]
pub type Pout04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT05` reader - desc POUT05"]
pub type Pout05R = crate::BitReader;
#[doc = "Field `POUT05` writer - desc POUT05"]
pub type Pout05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT06` reader - desc POUT06"]
pub type Pout06R = crate::BitReader;
#[doc = "Field `POUT06` writer - desc POUT06"]
pub type Pout06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT07` reader - desc POUT07"]
pub type Pout07R = crate::BitReader;
#[doc = "Field `POUT07` writer - desc POUT07"]
pub type Pout07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT08` reader - desc POUT08"]
pub type Pout08R = crate::BitReader;
#[doc = "Field `POUT08` writer - desc POUT08"]
pub type Pout08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT09` reader - desc POUT09"]
pub type Pout09R = crate::BitReader;
#[doc = "Field `POUT09` writer - desc POUT09"]
pub type Pout09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT10` reader - desc POUT10"]
pub type Pout10R = crate::BitReader;
#[doc = "Field `POUT10` writer - desc POUT10"]
pub type Pout10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT11` reader - desc POUT11"]
pub type Pout11R = crate::BitReader;
#[doc = "Field `POUT11` writer - desc POUT11"]
pub type Pout11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT12` reader - desc POUT12"]
pub type Pout12R = crate::BitReader;
#[doc = "Field `POUT12` writer - desc POUT12"]
pub type Pout12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT13` reader - desc POUT13"]
pub type Pout13R = crate::BitReader;
#[doc = "Field `POUT13` writer - desc POUT13"]
pub type Pout13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT14` reader - desc POUT14"]
pub type Pout14R = crate::BitReader;
#[doc = "Field `POUT14` writer - desc POUT14"]
pub type Pout14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT15` reader - desc POUT15"]
pub type Pout15R = crate::BitReader;
#[doc = "Field `POUT15` writer - desc POUT15"]
pub type Pout15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POUT00"]
    #[inline(always)]
    pub fn pout00(&self) -> Pout00R {
        Pout00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POUT01"]
    #[inline(always)]
    pub fn pout01(&self) -> Pout01R {
        Pout01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POUT02"]
    #[inline(always)]
    pub fn pout02(&self) -> Pout02R {
        Pout02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POUT03"]
    #[inline(always)]
    pub fn pout03(&self) -> Pout03R {
        Pout03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POUT04"]
    #[inline(always)]
    pub fn pout04(&self) -> Pout04R {
        Pout04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POUT05"]
    #[inline(always)]
    pub fn pout05(&self) -> Pout05R {
        Pout05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POUT06"]
    #[inline(always)]
    pub fn pout06(&self) -> Pout06R {
        Pout06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POUT07"]
    #[inline(always)]
    pub fn pout07(&self) -> Pout07R {
        Pout07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POUT08"]
    #[inline(always)]
    pub fn pout08(&self) -> Pout08R {
        Pout08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POUT09"]
    #[inline(always)]
    pub fn pout09(&self) -> Pout09R {
        Pout09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POUT10"]
    #[inline(always)]
    pub fn pout10(&self) -> Pout10R {
        Pout10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POUT11"]
    #[inline(always)]
    pub fn pout11(&self) -> Pout11R {
        Pout11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POUT12"]
    #[inline(always)]
    pub fn pout12(&self) -> Pout12R {
        Pout12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POUT13"]
    #[inline(always)]
    pub fn pout13(&self) -> Pout13R {
        Pout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POUT14"]
    #[inline(always)]
    pub fn pout14(&self) -> Pout14R {
        Pout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POUT15"]
    #[inline(always)]
    pub fn pout15(&self) -> Pout15R {
        Pout15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POUT00"]
    #[inline(always)]
    pub fn pout00(&mut self) -> Pout00W<'_, PodrbSpec> {
        Pout00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POUT01"]
    #[inline(always)]
    pub fn pout01(&mut self) -> Pout01W<'_, PodrbSpec> {
        Pout01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POUT02"]
    #[inline(always)]
    pub fn pout02(&mut self) -> Pout02W<'_, PodrbSpec> {
        Pout02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc POUT03"]
    #[inline(always)]
    pub fn pout03(&mut self) -> Pout03W<'_, PodrbSpec> {
        Pout03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc POUT04"]
    #[inline(always)]
    pub fn pout04(&mut self) -> Pout04W<'_, PodrbSpec> {
        Pout04W::new(self, 4)
    }
    #[doc = "Bit 5 - desc POUT05"]
    #[inline(always)]
    pub fn pout05(&mut self) -> Pout05W<'_, PodrbSpec> {
        Pout05W::new(self, 5)
    }
    #[doc = "Bit 6 - desc POUT06"]
    #[inline(always)]
    pub fn pout06(&mut self) -> Pout06W<'_, PodrbSpec> {
        Pout06W::new(self, 6)
    }
    #[doc = "Bit 7 - desc POUT07"]
    #[inline(always)]
    pub fn pout07(&mut self) -> Pout07W<'_, PodrbSpec> {
        Pout07W::new(self, 7)
    }
    #[doc = "Bit 8 - desc POUT08"]
    #[inline(always)]
    pub fn pout08(&mut self) -> Pout08W<'_, PodrbSpec> {
        Pout08W::new(self, 8)
    }
    #[doc = "Bit 9 - desc POUT09"]
    #[inline(always)]
    pub fn pout09(&mut self) -> Pout09W<'_, PodrbSpec> {
        Pout09W::new(self, 9)
    }
    #[doc = "Bit 10 - desc POUT10"]
    #[inline(always)]
    pub fn pout10(&mut self) -> Pout10W<'_, PodrbSpec> {
        Pout10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc POUT11"]
    #[inline(always)]
    pub fn pout11(&mut self) -> Pout11W<'_, PodrbSpec> {
        Pout11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc POUT12"]
    #[inline(always)]
    pub fn pout12(&mut self) -> Pout12W<'_, PodrbSpec> {
        Pout12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc POUT13"]
    #[inline(always)]
    pub fn pout13(&mut self) -> Pout13W<'_, PodrbSpec> {
        Pout13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POUT14"]
    #[inline(always)]
    pub fn pout14(&mut self) -> Pout14W<'_, PodrbSpec> {
        Pout14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POUT15"]
    #[inline(always)]
    pub fn pout15(&mut self) -> Pout15W<'_, PodrbSpec> {
        Pout15W::new(self, 15)
    }
}
#[doc = "desc PODRB\n\nYou can [`read`](crate::Reg::read) this register and get [`podrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PodrbSpec;
impl crate::RegisterSpec for PodrbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`podrb::R`](R) reader structure"]
impl crate::Readable for PodrbSpec {}
#[doc = "`write(|w| ..)` method takes [`podrb::W`](W) writer structure"]
impl crate::Writable for PodrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PODRB to value 0"]
impl crate::Resettable for PodrbSpec {}
