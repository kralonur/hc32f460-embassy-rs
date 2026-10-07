#[doc = "Register `PDWKE1` reader"]
pub type R = crate::R<Pdwke1Spec>;
#[doc = "Register `PDWKE1` writer"]
pub type W = crate::W<Pdwke1Spec>;
#[doc = "Field `WKE20` reader - desc WKE20"]
pub type Wke20R = crate::BitReader;
#[doc = "Field `WKE20` writer - desc WKE20"]
pub type Wke20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE21` reader - desc WKE21"]
pub type Wke21R = crate::BitReader;
#[doc = "Field `WKE21` writer - desc WKE21"]
pub type Wke21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE22` reader - desc WKE22"]
pub type Wke22R = crate::BitReader;
#[doc = "Field `WKE22` writer - desc WKE22"]
pub type Wke22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE23` reader - desc WKE23"]
pub type Wke23R = crate::BitReader;
#[doc = "Field `WKE23` writer - desc WKE23"]
pub type Wke23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE30` reader - desc WKE30"]
pub type Wke30R = crate::BitReader;
#[doc = "Field `WKE30` writer - desc WKE30"]
pub type Wke30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE31` reader - desc WKE31"]
pub type Wke31R = crate::BitReader;
#[doc = "Field `WKE31` writer - desc WKE31"]
pub type Wke31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE32` reader - desc WKE32"]
pub type Wke32R = crate::BitReader;
#[doc = "Field `WKE32` writer - desc WKE32"]
pub type Wke32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE33` reader - desc WKE33"]
pub type Wke33R = crate::BitReader;
#[doc = "Field `WKE33` writer - desc WKE33"]
pub type Wke33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc WKE20"]
    #[inline(always)]
    pub fn wke20(&self) -> Wke20R {
        Wke20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WKE21"]
    #[inline(always)]
    pub fn wke21(&self) -> Wke21R {
        Wke21R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc WKE22"]
    #[inline(always)]
    pub fn wke22(&self) -> Wke22R {
        Wke22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WKE23"]
    #[inline(always)]
    pub fn wke23(&self) -> Wke23R {
        Wke23R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WKE30"]
    #[inline(always)]
    pub fn wke30(&self) -> Wke30R {
        Wke30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WKE31"]
    #[inline(always)]
    pub fn wke31(&self) -> Wke31R {
        Wke31R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc WKE32"]
    #[inline(always)]
    pub fn wke32(&self) -> Wke32R {
        Wke32R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc WKE33"]
    #[inline(always)]
    pub fn wke33(&self) -> Wke33R {
        Wke33R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc WKE20"]
    #[inline(always)]
    pub fn wke20(&mut self) -> Wke20W<'_, Pdwke1Spec> {
        Wke20W::new(self, 0)
    }
    #[doc = "Bit 1 - desc WKE21"]
    #[inline(always)]
    pub fn wke21(&mut self) -> Wke21W<'_, Pdwke1Spec> {
        Wke21W::new(self, 1)
    }
    #[doc = "Bit 2 - desc WKE22"]
    #[inline(always)]
    pub fn wke22(&mut self) -> Wke22W<'_, Pdwke1Spec> {
        Wke22W::new(self, 2)
    }
    #[doc = "Bit 3 - desc WKE23"]
    #[inline(always)]
    pub fn wke23(&mut self) -> Wke23W<'_, Pdwke1Spec> {
        Wke23W::new(self, 3)
    }
    #[doc = "Bit 4 - desc WKE30"]
    #[inline(always)]
    pub fn wke30(&mut self) -> Wke30W<'_, Pdwke1Spec> {
        Wke30W::new(self, 4)
    }
    #[doc = "Bit 5 - desc WKE31"]
    #[inline(always)]
    pub fn wke31(&mut self) -> Wke31W<'_, Pdwke1Spec> {
        Wke31W::new(self, 5)
    }
    #[doc = "Bit 6 - desc WKE32"]
    #[inline(always)]
    pub fn wke32(&mut self) -> Wke32W<'_, Pdwke1Spec> {
        Wke32W::new(self, 6)
    }
    #[doc = "Bit 7 - desc WKE33"]
    #[inline(always)]
    pub fn wke33(&mut self) -> Wke33W<'_, Pdwke1Spec> {
        Wke33W::new(self, 7)
    }
}
#[doc = "desc PDWKE1\n\nYou can [`read`](crate::Reg::read) this register and get [`pdwke1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdwke1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdwke1Spec;
impl crate::RegisterSpec for Pdwke1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pdwke1::R`](R) reader structure"]
impl crate::Readable for Pdwke1Spec {}
#[doc = "`write(|w| ..)` method takes [`pdwke1::W`](W) writer structure"]
impl crate::Writable for Pdwke1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDWKE1 to value 0"]
impl crate::Resettable for Pdwke1Spec {}
