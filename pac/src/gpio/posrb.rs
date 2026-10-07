#[doc = "Register `POSRB` reader"]
pub type R = crate::R<PosrbSpec>;
#[doc = "Register `POSRB` writer"]
pub type W = crate::W<PosrbSpec>;
#[doc = "Field `POS00` reader - desc POS00"]
pub type Pos00R = crate::BitReader;
#[doc = "Field `POS00` writer - desc POS00"]
pub type Pos00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS01` reader - desc POS01"]
pub type Pos01R = crate::BitReader;
#[doc = "Field `POS01` writer - desc POS01"]
pub type Pos01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS02` reader - desc POS02"]
pub type Pos02R = crate::BitReader;
#[doc = "Field `POS02` writer - desc POS02"]
pub type Pos02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS03` reader - desc POS03"]
pub type Pos03R = crate::BitReader;
#[doc = "Field `POS03` writer - desc POS03"]
pub type Pos03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS04` reader - desc POS04"]
pub type Pos04R = crate::BitReader;
#[doc = "Field `POS04` writer - desc POS04"]
pub type Pos04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS05` reader - desc POS05"]
pub type Pos05R = crate::BitReader;
#[doc = "Field `POS05` writer - desc POS05"]
pub type Pos05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS06` reader - desc POS06"]
pub type Pos06R = crate::BitReader;
#[doc = "Field `POS06` writer - desc POS06"]
pub type Pos06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS07` reader - desc POS07"]
pub type Pos07R = crate::BitReader;
#[doc = "Field `POS07` writer - desc POS07"]
pub type Pos07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS08` reader - desc POS08"]
pub type Pos08R = crate::BitReader;
#[doc = "Field `POS08` writer - desc POS08"]
pub type Pos08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS09` reader - desc POS09"]
pub type Pos09R = crate::BitReader;
#[doc = "Field `POS09` writer - desc POS09"]
pub type Pos09W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS10` reader - desc POS10"]
pub type Pos10R = crate::BitReader;
#[doc = "Field `POS10` writer - desc POS10"]
pub type Pos10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS11` reader - desc POS11"]
pub type Pos11R = crate::BitReader;
#[doc = "Field `POS11` writer - desc POS11"]
pub type Pos11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS12` reader - desc POS12"]
pub type Pos12R = crate::BitReader;
#[doc = "Field `POS12` writer - desc POS12"]
pub type Pos12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS13` reader - desc POS13"]
pub type Pos13R = crate::BitReader;
#[doc = "Field `POS13` writer - desc POS13"]
pub type Pos13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS14` reader - desc POS14"]
pub type Pos14R = crate::BitReader;
#[doc = "Field `POS14` writer - desc POS14"]
pub type Pos14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS15` reader - desc POS15"]
pub type Pos15R = crate::BitReader;
#[doc = "Field `POS15` writer - desc POS15"]
pub type Pos15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POS00"]
    #[inline(always)]
    pub fn pos00(&self) -> Pos00R {
        Pos00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc POS01"]
    #[inline(always)]
    pub fn pos01(&self) -> Pos01R {
        Pos01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc POS02"]
    #[inline(always)]
    pub fn pos02(&self) -> Pos02R {
        Pos02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc POS03"]
    #[inline(always)]
    pub fn pos03(&self) -> Pos03R {
        Pos03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc POS04"]
    #[inline(always)]
    pub fn pos04(&self) -> Pos04R {
        Pos04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc POS05"]
    #[inline(always)]
    pub fn pos05(&self) -> Pos05R {
        Pos05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc POS06"]
    #[inline(always)]
    pub fn pos06(&self) -> Pos06R {
        Pos06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc POS07"]
    #[inline(always)]
    pub fn pos07(&self) -> Pos07R {
        Pos07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc POS08"]
    #[inline(always)]
    pub fn pos08(&self) -> Pos08R {
        Pos08R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc POS09"]
    #[inline(always)]
    pub fn pos09(&self) -> Pos09R {
        Pos09R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc POS10"]
    #[inline(always)]
    pub fn pos10(&self) -> Pos10R {
        Pos10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc POS11"]
    #[inline(always)]
    pub fn pos11(&self) -> Pos11R {
        Pos11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc POS12"]
    #[inline(always)]
    pub fn pos12(&self) -> Pos12R {
        Pos12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc POS13"]
    #[inline(always)]
    pub fn pos13(&self) -> Pos13R {
        Pos13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POS14"]
    #[inline(always)]
    pub fn pos14(&self) -> Pos14R {
        Pos14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POS15"]
    #[inline(always)]
    pub fn pos15(&self) -> Pos15R {
        Pos15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POS00"]
    #[inline(always)]
    pub fn pos00(&mut self) -> Pos00W<'_, PosrbSpec> {
        Pos00W::new(self, 0)
    }
    #[doc = "Bit 1 - desc POS01"]
    #[inline(always)]
    pub fn pos01(&mut self) -> Pos01W<'_, PosrbSpec> {
        Pos01W::new(self, 1)
    }
    #[doc = "Bit 2 - desc POS02"]
    #[inline(always)]
    pub fn pos02(&mut self) -> Pos02W<'_, PosrbSpec> {
        Pos02W::new(self, 2)
    }
    #[doc = "Bit 3 - desc POS03"]
    #[inline(always)]
    pub fn pos03(&mut self) -> Pos03W<'_, PosrbSpec> {
        Pos03W::new(self, 3)
    }
    #[doc = "Bit 4 - desc POS04"]
    #[inline(always)]
    pub fn pos04(&mut self) -> Pos04W<'_, PosrbSpec> {
        Pos04W::new(self, 4)
    }
    #[doc = "Bit 5 - desc POS05"]
    #[inline(always)]
    pub fn pos05(&mut self) -> Pos05W<'_, PosrbSpec> {
        Pos05W::new(self, 5)
    }
    #[doc = "Bit 6 - desc POS06"]
    #[inline(always)]
    pub fn pos06(&mut self) -> Pos06W<'_, PosrbSpec> {
        Pos06W::new(self, 6)
    }
    #[doc = "Bit 7 - desc POS07"]
    #[inline(always)]
    pub fn pos07(&mut self) -> Pos07W<'_, PosrbSpec> {
        Pos07W::new(self, 7)
    }
    #[doc = "Bit 8 - desc POS08"]
    #[inline(always)]
    pub fn pos08(&mut self) -> Pos08W<'_, PosrbSpec> {
        Pos08W::new(self, 8)
    }
    #[doc = "Bit 9 - desc POS09"]
    #[inline(always)]
    pub fn pos09(&mut self) -> Pos09W<'_, PosrbSpec> {
        Pos09W::new(self, 9)
    }
    #[doc = "Bit 10 - desc POS10"]
    #[inline(always)]
    pub fn pos10(&mut self) -> Pos10W<'_, PosrbSpec> {
        Pos10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc POS11"]
    #[inline(always)]
    pub fn pos11(&mut self) -> Pos11W<'_, PosrbSpec> {
        Pos11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc POS12"]
    #[inline(always)]
    pub fn pos12(&mut self) -> Pos12W<'_, PosrbSpec> {
        Pos12W::new(self, 12)
    }
    #[doc = "Bit 13 - desc POS13"]
    #[inline(always)]
    pub fn pos13(&mut self) -> Pos13W<'_, PosrbSpec> {
        Pos13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POS14"]
    #[inline(always)]
    pub fn pos14(&mut self) -> Pos14W<'_, PosrbSpec> {
        Pos14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POS15"]
    #[inline(always)]
    pub fn pos15(&mut self) -> Pos15W<'_, PosrbSpec> {
        Pos15W::new(self, 15)
    }
}
#[doc = "desc POSRB\n\nYou can [`read`](crate::Reg::read) this register and get [`posrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosrbSpec;
impl crate::RegisterSpec for PosrbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`posrb::R`](R) reader structure"]
impl crate::Readable for PosrbSpec {}
#[doc = "`write(|w| ..)` method takes [`posrb::W`](W) writer structure"]
impl crate::Writable for PosrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POSRB to value 0"]
impl crate::Resettable for PosrbSpec {}
